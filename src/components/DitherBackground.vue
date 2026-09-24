<template>
	<canvas
		ref="shaderCanvas"
		class="dither-shader-canvas"
		:style="{
			filter: `blur(${scrollProgress * 8}px) brightness(${1 - scrollProgress * 0.45})`,
			opacity: (1 - scrollProgress * 0.45) * 0.40,
		}"
		aria-hidden="true"
	></canvas>
</template>

<script>
export default {
	name: 'DitherBackground',
	data() {
		return {
			scrollProgress: 0,
		};
	},
	mounted() {
		this.initShader();
		this.initScrollListener();
	},
	beforeUnmount() {
		if (this.cleanup) {
			this.cleanup();
		}
	},
	methods: {
		initScrollListener() {
			let scrollTicking = false;
			const onScroll = () => {
				if (!scrollTicking) {
					window.requestAnimationFrame(() => {
						const scrollY = window.scrollY || 0;
						const threshold = Math.max(window.innerHeight * 0.55, 300);
						this.scrollProgress = Math.min(Math.max(scrollY / threshold, 0), 1);
						scrollTicking = false;
					});
					scrollTicking = true;
				}
			};
			window.addEventListener('scroll', onScroll, { passive: true });
			this._removeScroll = () => window.removeEventListener('scroll', onScroll);
		},

		createShader(gl, type, source) {
			const shader = gl.createShader(type);
			gl.shaderSource(shader, source);
			gl.compileShader(shader);

			if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
				console.error(gl.getShaderInfoLog(shader));
				gl.deleteShader(shader);
				return null;
			}
			return shader;
		},

		createProgram(gl, vertexSource, fragmentSource) {
			const vertexShader = this.createShader(gl, gl.VERTEX_SHADER, vertexSource);
			const fragmentShader = this.createShader(gl, gl.FRAGMENT_SHADER, fragmentSource);

			if (!vertexShader || !fragmentShader) return null;

			const program = gl.createProgram();
			gl.attachShader(program, vertexShader);
			gl.attachShader(program, fragmentShader);
			gl.linkProgram(program);
			gl.deleteShader(vertexShader);
			gl.deleteShader(fragmentShader);

			if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
				console.error(gl.getProgramInfoLog(program));
				gl.deleteProgram(program);
				return null;
			}
			return program;
		},

		initShader() {
			const canvas = this.$refs.shaderCanvas;
			const reduceMotionQuery = window.matchMedia?.('(prefers-reduced-motion: reduce)');

			if (!canvas || reduceMotionQuery?.matches) return;

			const gl = canvas.getContext('webgl', {
				antialias: false,
				alpha: true,
				depth: false,
				stencil: false,
				preserveDrawingBuffer: false,
				powerPreference: 'low-power',
			});

			if (!gl) return;

			const vertexSource = `
				attribute vec2 a_position;
				varying vec2 v_uv;

				void main() {
					v_uv = a_position * 0.5 + 0.5;
					gl_Position = vec4(a_position, 0.0, 1.0);
				}
			`;

			const fragmentSource = `
				precision mediump float;

				uniform vec2 u_resolution;
				uniform float u_time;
				varying vec2 v_uv;

				// 4x4 Bayer Dither Matrix for retro ASCII/pixel art shading
				float getBayer4(vec2 coord) {
					vec2 b = mod(coord, 4.0);
					float x = floor(b.x);
					float y = floor(b.y);
					if (y == 0.0) {
						return (x == 0.0 ? 0.0 : x == 1.0 ? 8.0 : x == 2.0 ? 2.0 : 10.0) / 16.0;
					} else if (y == 1.0) {
						return (x == 0.0 ? 12.0 : x == 1.0 ? 4.0 : x == 2.0 ? 14.0 : 6.0) / 16.0;
					} else if (y == 2.0) {
						return (x == 0.0 ? 3.0 : x == 1.0 ? 11.0 : x == 2.0 ? 1.0 : 9.0) / 16.0;
					} else {
						return (x == 0.0 ? 15.0 : x == 1.0 ? 7.0 : x == 2.0 ? 13.0 : 5.0) / 16.0;
					}
				}

				float hash(vec2 p) {
					return fract(sin(dot(p, vec2(120.1, 311.7))) * 758.5453123);
				}

				float noise(vec2 p) {
					vec2 i = floor(p);
					vec2 f = fract(p);
					vec2 u = f * f * (3.0 - 2.0 * f);

					float a = hash(i);
					float b = hash(i + vec2(1.0, 0.0));
					float c = hash(i + vec2(0.0, 1.0));
					float d = hash(i + vec2(1.0, 1.0));

					return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
				}

				float fbm(vec2 p) {
					float value = 0.0;
					float amplitude = 0.5;
					mat2 rot = mat2(0.80, -0.60, 0.60, 0.80);

					for (int i = 0; i < 3; i++) {
						value += amplitude * noise(p);
						p = rot * p * 2.04 + vec2(11.7, 4.3);
						amplitude *= 0.5;
					}

					return value;
				}

				vec2 flowField(vec2 p, float t) {
					float n1 = fbm(p * 1.15 + vec2(t * 0.08, -t * 0.05));
					float n2 = fbm(p * 1.10 + vec2(-t * 0.06, t * 0.09) + 7.3);
					float angle = (n1 - n2) * 6.28318 + t * 0.16;
					return vec2(cos(angle), sin(angle));
				}

				void main() {
					// Pixel art block size (retro pixel dither size)
					float pixelSize = 9.0;
					vec2 pixelCoord = floor(gl_FragCoord.xy / pixelSize);
					vec2 snappedCoord = (pixelCoord + 0.5) * pixelSize;

					vec2 uv = snappedCoord / u_resolution.xy;
					vec2 p = uv - 0.5;
					p.x *= u_resolution.x / u_resolution.y;
					float t = u_time;

					vec2 q = p;
					for (int i = 0; i < 2; i++) {
						vec2 flow = flowField(q * 0.9, t);
						q += flow * 0.05;
						q += vec2(sin(t * 0.11 + q.y * 2.0), cos(t * 0.10 + q.x * 2.0)) * 0.025;
					}

					vec2 drift = vec2(t * 0.035, -t * 0.022);
					float smoke = fbm(q * 2.0 + drift);
					smoke += 0.55 * fbm(q * 4.0 - drift.yx + 2.7);
					smoke = smoke / 0.75;

					float haze = fbm(q * 0.78 + vec2(-t * 0.018, t * 0.014));
					float rawDensity = haze * 0.40 + smoke * 0.60;

					// High-contrast curve for punchy pixel clusters
					float shapedDensity = smoothstep(0.12, 0.86, rawDensity);
					shapedDensity = pow(shapedDensity, 1.15);

					// 4x4 Bayer matrix ordered dithering
					float dither = (getBayer4(pixelCoord) - 0.5) * 0.22;
					float dithered = clamp(shapedDensity + dither, 0.0, 1.0);

					// 6 discrete monochrome palette steps
					float level = floor(dithered * 6.0) / 6.0;
					float edgeFalloff = 0.88 + 0.12 * smoothstep(1.4, 0.1, length(p));

					// Deep Muted Shadow Palette:
					// Strictly limited to dark gray tones (max 22% luminance)
					// to guarantee 100% separation from crisp white foreground text
					vec3 c0 = vec3(0.020, 0.020, 0.020); // Deep Void (#050505)
					vec3 c1 = vec3(0.035, 0.035, 0.035); // Charcoal (#090909)
					vec3 c2 = vec3(0.060, 0.060, 0.060); // Slate (#0f0f0f)
					vec3 c3 = vec3(0.095, 0.095, 0.095); // Deep Ash (#181818)
					vec3 c4 = vec3(0.145, 0.145, 0.145); // Graphite (#252525)
					vec3 c5 = vec3(0.220, 0.220, 0.220); // Muted Silver (#383838, max peak!)

					vec3 color = c0;
					float alpha = 0.0;

					if (level < 0.166) {
						color = c0;
						alpha = mix(0.15, 0.25, level * 6.0);
					} else if (level < 0.333) {
						color = c1;
						alpha = mix(0.25, 0.38, (level - 0.166) * 6.0);
					} else if (level < 0.500) {
						color = c2;
						alpha = mix(0.38, 0.50, (level - 0.333) * 6.0);
					} else if (level < 0.666) {
						color = c3;
						alpha = mix(0.50, 0.62, (level - 0.500) * 6.0);
					} else if (level < 0.833) {
						color = c4;
						alpha = mix(0.62, 0.72, (level - 0.666) * 6.0);
					} else {
						color = c5;
						alpha = 0.78;
					}

					// Gentle subtle pixel sparkles (Muted, never blinding white)
					vec2 starGrid = floor((pixelCoord + vec2(t * 1.2, -t * 1.8)) / 11.0);
					float starSeed = hash(starGrid * 5.43);
					if (starSeed > 0.975) {
						vec2 localGrid = mod(pixelCoord + vec2(t * 1.2, -t * 1.8), 11.0);
						if (floor(localGrid.x) == 5.0 && floor(localGrid.y) == 5.0) {
							float pulse = sin(t * 3.5 + starSeed * 50.0) * 0.5 + 0.5;
							if (pulse > 0.4) {
								vec3 sparkColor = mix(c4, vec3(0.30), pulse);
								color = mix(color, sparkColor, pulse);
							}
						}
					}

					// CRT/LCD pixel grid texture
					vec2 frac = fract(gl_FragCoord.xy / pixelSize);
					float grid = (frac.x < 0.06 || frac.y < 0.06) ? 0.90 : 1.0;
					color *= grid;

					// Central Reading Corridor Mask (Learned from QCommunity):
					// Significantly softens the background right under the central reading corridor
					float readingZone = smoothstep(0.0, 0.88, length(vec2(p.x * 0.85, p.y * 1.35)));
					alpha *= mix(0.20, 1.0, readingZone);

					alpha *= edgeFalloff * 0.75;

					gl_FragColor = vec4(color, alpha);
				}
			`;

			const program = this.createProgram(gl, vertexSource, fragmentSource);
			if (!program) return;

			const positionBuffer = gl.createBuffer();
			gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
			gl.bufferData(
				gl.ARRAY_BUFFER,
				new Float32Array([-1, -1, 1, -1, -1, 1, -1, 1, 1, -1, 1, 1]),
				gl.STATIC_DRAW
			);

			const positionLocation = gl.getAttribLocation(program, 'a_position');
			const resolutionLocation = gl.getUniformLocation(program, 'u_resolution');
			const timeLocation = gl.getUniformLocation(program, 'u_time');

			const startTime = performance.now();
			let animationFrame = 0;
			let isCanvasVisible = true;
			let shouldRender = true;
			let isLoopRunning = false;
			let needsResize = true;
			let lastFrameTime = 0;
			const targetInterval = 1000 / 36; // 36 FPS limit for optimal battery & performance

			const resize = () => {
				const dpr = Math.min(window.devicePixelRatio || 1, 1.25);
				const width = Math.max(1, Math.floor(canvas.clientWidth * dpr));
				const height = Math.max(1, Math.floor(canvas.clientHeight * dpr));

				if (canvas.width !== width || canvas.height !== height) {
					canvas.width = width;
					canvas.height = height;
					gl.viewport(0, 0, width, height);
				}
			};

			const requestResize = () => {
				needsResize = true;
			};

			const renderFrame = () => {
				if (needsResize) {
					resize();
					needsResize = false;
				}

				gl.clearColor(0, 0, 0, 0);
				gl.clear(gl.COLOR_BUFFER_BIT);
				gl.useProgram(program);
				gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);
				gl.enableVertexAttribArray(positionLocation);
				gl.vertexAttribPointer(positionLocation, 2, gl.FLOAT, false, 0, 0);
				gl.uniform2f(resolutionLocation, canvas.width, canvas.height);

				const elapsed = (performance.now() - startTime) * 0.001 * 1.8;
				gl.uniform1f(timeLocation, elapsed);

				gl.drawArrays(gl.TRIANGLES, 0, 6);
			};

			const render = (now) => {
				if (!shouldRender) {
					isLoopRunning = false;
					return;
				}

				animationFrame = requestAnimationFrame(render);

				if (now - lastFrameTime >= targetInterval) {
					lastFrameTime = now;
					renderFrame();
				}
			};

			const startRenderLoop = () => {
				if (!isLoopRunning) {
					isLoopRunning = true;
					animationFrame = requestAnimationFrame(render);
				}
			};

			const updateRenderState = () => {
				shouldRender = document.visibilityState === 'visible' && isCanvasVisible;
				if (shouldRender) {
					startRenderLoop();
				} else {
					cancelAnimationFrame(animationFrame);
					isLoopRunning = false;
				}
			};

			const visibilityObserver = new IntersectionObserver(
				([entry]) => {
					isCanvasVisible = entry.isIntersecting;
					updateRenderState();
				},
				{ threshold: 0 }
			);

			window.addEventListener('resize', requestResize, { passive: true });
			document.addEventListener('visibilitychange', updateRenderState);
			visibilityObserver.observe(canvas);

			renderFrame();
			startRenderLoop();

			this.cleanup = () => {
				cancelAnimationFrame(animationFrame);
				if (this._removeScroll) this._removeScroll();
				window.removeEventListener('resize', requestResize);
				document.removeEventListener('visibilitychange', updateRenderState);
				visibilityObserver.disconnect();
				gl.deleteBuffer(positionBuffer);
				gl.deleteProgram(program);
			};
		},
	},
};
</script>

<style scoped>
.dither-shader-canvas {
	position: fixed;
	top: 0;
	left: 0;
	width: 100vw;
	height: 100vh;
	pointer-events: none;
	z-index: 0;
	image-rendering: -webkit-optimize-contrast;
	image-rendering: crisp-edges;
	image-rendering: pixelated;
	transition: filter 180ms ease-out, opacity 180ms ease-out;
}

@media (prefers-reduced-motion: reduce) {
	.dither-shader-canvas {
		display: none;
	}
}
</style>
