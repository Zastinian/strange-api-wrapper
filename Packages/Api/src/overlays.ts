import type core from "@hedystia/strange.core";

/**
 * @class
 */
export default class Overlays {
	core: core;
	constructor(core: core) {
		this.core = core;
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	approved({ image }: { image: string }): ArrayBuffer {
		return this.core.approved({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	brazzers({ image }: { image: string }): ArrayBuffer {
		return this.core.brazzers({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	gay({ image }: { image: string }): ArrayBuffer {
		return this.core.gay({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	halloween({ image }: { image: string }): ArrayBuffer {
		return this.core.halloween({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	rejected({ image }: { image: string }): ArrayBuffer {
		return this.core.rejected({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	thuglife({ image }: { image: string }): ArrayBuffer {
		return this.core.thuglife({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	toBeContinued({ image }: { image: string }): ArrayBuffer {
		return this.core.toBeContinued({
			image,
		});
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	wasted({ image }: { image: string }): ArrayBuffer {
		return this.core.wasted({
			image,
		});
	}
}
