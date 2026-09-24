/**
 * TextScramble Utility
 * Creates a hacker/terminal style character decode effect on hover.
 */
export class TextScramble {
	constructor(el, chars = '!<>-_\\/[]{}—=+*^?#01') {
		this.el = el;
		this.chars = chars;
		this.originalText = el.innerText;
		this.frame = 0;
		this.queue = [];
		this.frameRequest = null;
	}

	setText(newText) {
		const oldText = this.el.innerText;
		const length = Math.max(oldText.length, newText.length);
		this.queue = [];

		for (let i = 0; i < length; i++) {
			const from = oldText[i] || '';
			const to = newText[i] || '';
			const start = Math.floor(Math.random() * 8);
			const end = start + Math.floor(Math.random() * 12);
			this.queue.push({ from, to, start, end, char: '' });
		}

		cancelAnimationFrame(this.frameRequest);
		this.frame = 0;
		return new Promise((resolve) => {
			this.resolve = resolve;
			this.update();
		});
	}

	update() {
		let output = '';
		let complete = 0;

		for (let i = 0, n = this.queue.length; i < n; i++) {
			let { from, to, start, end, char } = this.queue[i];
			if (this.frame >= end) {
				complete++;
				output += to;
			} else if (this.frame >= start) {
				if (!char || Math.random() < 0.28) {
					char = this.chars[Math.floor(Math.random() * this.chars.length)];
					this.queue[i].char = char;
				}
				output += `<span class="scramble-glyph">${char}</span>`;
			} else {
				output += from;
			}
		}

		this.el.innerHTML = output;

		if (complete === this.queue.length) {
			this.resolve();
		} else {
			this.frame++;
			this.frameRequest = requestAnimationFrame(() => this.update());
		}
	}

	destroy() {
		cancelAnimationFrame(this.frameRequest);
		this.el.innerText = this.originalText;
	}
}

/**
 * Helper to attach scramble effect on hover to an element.
 */
export function attachScramble(el) {
	if (!el) return () => {};
	const original = el.innerText;
	const scrambler = new TextScramble(el);
	let isRunning = false;

	const onMouseEnter = () => {
		if (isRunning) return;
		isRunning = true;
		scrambler.setText(original).then(() => {
			isRunning = false;
		});
	};

	el.addEventListener('mouseenter', onMouseEnter);

	return () => {
		el.removeEventListener('mouseenter', onMouseEnter);
		scrambler.destroy();
	};
}
