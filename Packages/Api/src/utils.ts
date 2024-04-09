import type core from "@hedystia/strange.core"

/**
 * @class
 */
export default class Utils {
	core: core
	constructor(core: core) {
		this.core = core
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	circle({ image }: { image: string }): ArrayBuffer {
		return this.core.circle({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.code
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	color({ code }: { code: string }): ArrayBuffer {
		return this.core.color({
			code,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	denoise({ image }: { image: string }): ArrayBuffer {
		return this.core.denoise({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.avatar
	 * @param {string} options.name
	 * @param {string} options.discriminator
	 * @param {number} options.count
	 * @param {string} options.guild
	 * @param {string} options.bkg
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	farewellCard({
		avatar,
		name,
		discriminator,
		count,
		guild,
		bkg,
	}: {
		avatar: string
		name: string
		discriminator: string
		count: number
		guild: string
		bkg: string
	}): ArrayBuffer {
		return this.core.farewellCard({
			avatar,
			name,
			discriminator,
			count,
			guild,
			bkg,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.avatar
	 * @param {number} options.currentxp
	 * @param {number} options.reqxp
	 * @param {number} options.level
	 * @param {number} options.rank
	 * @param {string} options.status
	 * @param {string} options.name
	 * @param {string} options.discriminator
	 * @param {string} options.barcolor
	 * @param {string} options.bgimage
	 * @param {string} options.bgcolor
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	rankCard({
		avatar,
		currentxp,
		reqxp,
		level,
		rank,
		status,
		name,
		discriminator,
		barcolor,
		bgimage,
		bgcolor,
	}: {
		avatar: string
		currentxp: number
		reqxp: number
		level: number
		rank: number
		status: string
		name: string
		discriminator: string
		barcolor: string
		bgimage: string
		bgcolor: string
	}): ArrayBuffer {
		return this.core.rankCard({
			avatar,
			currentxp,
			reqxp,
			level,
			rank,
			status,
			name,
			discriminator,
			barcolor,
			bgimage,
			bgcolor,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {string} options.author
	 * @param {string} options.album
	 * @param {number} options.start
	 * @param {number} options.end
	 * @param {string} options.title
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	spotifyCard({
		image,
		author,
		album,
		start,
		end,
		title,
	}: {
		image: string
		author: string
		album: string
		start: number
		end: number
		title: string
	}): ArrayBuffer {
		return this.core.spotifyCard({
			image,
			author,
			album,
			start,
			end,
			title,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.avatar
	 * @param {string} options.name
	 * @param {string} options.discriminator
	 * @param {number} options.count
	 * @param {string} options.guild
	 * @param {string} options.bkg
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	welcomeCard({
		avatar,
		name,
		discriminator,
		count,
		guild,
		bkg,
	}: {
		avatar: string
		name: string
		discriminator: string
		count: number
		guild: string
		bkg: string
	}): ArrayBuffer {
		return this.core.welcomeCard({
			avatar,
			name,
			discriminator,
			count,
			guild,
			bkg,
		})
	}
}
