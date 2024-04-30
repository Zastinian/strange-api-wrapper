import type core from "@hedystia/strange.core";

/**
 * @class
 */
export default class Filters {
	core: core;
	constructor(core: core) {
		this.core = core;
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.level
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	blur({ image, level }: { image: string; level: number }): ArrayBuffer {
		return this.core.blur({
			image,
			level,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.amount
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	brighten({ image, amount }: { image: string; amount: number }): ArrayBuffer {
		return this.core.brighten({
			image,
			amount,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.level
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	burn({ image, level }: { image: string; level: number }): ArrayBuffer {
		return this.core.burn({
			image,
			level,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.amount
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	darken({ image, amount }: { image: string; amount: number }): ArrayBuffer {
		return this.core.darken({
			image,
			amount,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	deepfry({ image }: { image: string }): ArrayBuffer {
		return this.core.deepfry({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.level
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	distort({ image, level }: { image: string; level: number }): ArrayBuffer {
		return this.core.distort({
			image,
			level,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	greyscale({ image }: { image: string }): ArrayBuffer {
		return this.core.greyscale({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	invert({ image }: { image: string }): ArrayBuffer {
		return this.core.invert({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.pixels
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	pixelate({ image, pixels }: { image: string; pixels: number }): ArrayBuffer {
		return this.core.pixelate({
			image,
			pixels,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	sepia({ image }: { image: string }): ArrayBuffer {
		return this.core.sepia({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.level
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	sharpen({ image, level }: { image: string; level: number }): ArrayBuffer {
		return this.core.sharpen({
			image,
			level,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {number} options.amount
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	threshold({ image, amount }: { image: string; amount: number }): ArrayBuffer {
		return this.core.threshold({
			image,
			amount,
		});
	}
}
