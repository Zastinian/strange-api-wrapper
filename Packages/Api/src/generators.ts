import type core from "../../Core"

/**
 * @class
 */
export default class Generators {
	core: core
	constructor(core: core) {
		this.core = core
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	pokemon3000Years({ image }: { image: string }): ArrayBuffer {
		return this.core.pokemon3000Years({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	ad({ image }: { image: string }): ArrayBuffer {
		return this.core.ad({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	affect({ image }: { image: string }): ArrayBuffer {
		return this.core.affect({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image1
	 * @param {string} options.image2
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	batslap({ image1, image2 }: { image1: string; image2: string }): ArrayBuffer {
		return this.core.batslap({
			image1,
			image2,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	beautiful({ image }: { image: string }): ArrayBuffer {
		return this.core.beautiful({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image1
	 * @param {string} options.image2
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	bed({ image1, image2 }: { image1: string; image2: string }): ArrayBuffer {
		return this.core.bed({
			image1,
			image2,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	bobross({ image }: { image: string }): ArrayBuffer {
		return this.core.bobross({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {boolean} options.silhouetted
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	challenger({ image, silhouetted }: { image: string; silhouetted: boolean }): ArrayBuffer {
		return this.core.challenger({
			image,
			silhouetted,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.text
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	changemymind({ text }: { text: string }): ArrayBuffer {
		return this.core.changemymind({
			text,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	clown({ image }: { image: string }): ArrayBuffer {
		return this.core.clown({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.text
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	clyde({ text }: { text: string }): ArrayBuffer {
		return this.core.clyde({
			text,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	confusedstonk({ image }: { image: string }): ArrayBuffer {
		return this.core.confusedstonk({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	delete({ image }: { image: string }): ArrayBuffer {
		return this.core.delete({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.title
	 * @param {string} options.text
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	demotivational({
		title,
		text,
		image,
	}: {
		title: string
		text: string
		image: string
	}): ArrayBuffer {
		return this.core.demotivational({
			title,
			text,
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	dexter({ image }: { image: string }): ArrayBuffer {
		return this.core.dexter({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	discordblack({ image }: { image: string }): ArrayBuffer {
		return this.core.discordblack({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	discordblue({ image }: { image: string }): ArrayBuffer {
		return this.core.discordblue({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image1
	 * @param {string} options.image2
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	doublestonk({ image1, image2 }: { image1: string; image2: string }): ArrayBuffer {
		return this.core.doublestonk({
			image1,
			image2,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	facepalm({ image }: { image: string }): ArrayBuffer {
		return this.core.facepalm({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image1
	 * @param {string} options.image2
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	fusion({ image1, image2 }: { image1: string; image2: string }): ArrayBuffer {
		return this.core.fusion({
			image1,
			image2,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.first_step
	 * @param {string} options.second_step
	 * @param {string} options.third_step
	 * @param {string} options.fourth_step
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	gruPlan({
		first_step,
		second_step,
		third_step,
		fourth_step,
	}: {
		first_step: string
		second_step: string
		third_step: string
		fourth_step: string
	}): ArrayBuffer {
		return this.core.gruPlan({
			first_step,
			second_step,
			third_step,
			fourth_step,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	heartbreaking({ image }: { image: string }): ArrayBuffer {
		return this.core.heartbreaking({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	hitler({ image }: { image: string }): ArrayBuffer {
		return this.core.hitler({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	jail({ image }: { image: string }): ArrayBuffer {
		return this.core.jail({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	jokeoverhead({ image }: { image: string }): ArrayBuffer {
		return this.core.jokeoverhead({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	karaba({ image }: { image: string }): ArrayBuffer {
		return this.core.karaba({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image1
	 * @param {string} options.image2
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	kiss({ image1, image2 }: { image1: string; image2: string }): ArrayBuffer {
		return this.core.kiss({
			image1,
			image2,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	kyonGun({ image }: { image: string }): ArrayBuffer {
		return this.core.kyonGun({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.text
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	lisaPresentation({ text }: { text: string }): ArrayBuffer {
		return this.core.lisaPresentation({
			text,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	mikkelsen({ image }: { image: string }): ArrayBuffer {
		return this.core.mikkelsen({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	mirror({ image }: { image: string }): ArrayBuffer {
		return this.core.mirror({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	mms({ image }: { image: string }): ArrayBuffer {
		return this.core.mms({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	notstonk({ image }: { image: string }): ArrayBuffer {
		return this.core.notstonk({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.text
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	ohno({ text }: { text: string }): ArrayBuffer {
		return this.core.ohno({
			text,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.first_step
	 * @param {string} options.second_step
	 * @param {string} options.third_step
	 * @param {string} options.fourth_step
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	planktonPlan({
		first_step,
		second_step,
		third_step,
		fourth_step,
	}: {
		first_step: string
		second_step: string
		third_step: string
		fourth_step: string
	}): ArrayBuffer {
		return this.core.planktonPlan({
			first_step,
			second_step,
			third_step,
			fourth_step,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	poutine({ image }: { image: string }): ArrayBuffer {
		return this.core.poutine({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	rip({ image }: { image: string }): ArrayBuffer {
		return this.core.rip({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	shit({ image }: { image: string }): ArrayBuffer {
		return this.core.shit({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	snyder({ image }: { image: string }): ArrayBuffer {
		return this.core.snyder({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image1
	 * @param {string} options.image2
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	spank({ image1, image2 }: { image1: string; image2: string }): ArrayBuffer {
		return this.core.spank({
			image1,
			image2,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	stonk({ image }: { image: string }): ArrayBuffer {
		return this.core.stonk({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	tattoo({ image }: { image: string }): ArrayBuffer {
		return this.core.tattoo({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	thomas({ image }: { image: string }): ArrayBuffer {
		return this.core.thomas({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	trash({ image }: { image: string }): ArrayBuffer {
		return this.core.trash({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	wanted({ image }: { image: string }): ArrayBuffer {
		return this.core.wanted({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	worthless({ image }: { image: string }): ArrayBuffer {
		return this.core.worthless({
			image,
		})
	}

	/**
	 * @param {object} options
	 * @param {string} options.image
	 * @param {string} options.username
	 * @param {string} options.text
	 * @returns {ArrayBuffer} ArrayBuffer
	 */
	youtube({
		image,
		username,
		text,
	}: {
		image: string
		username: string
		text: string
	}): ArrayBuffer {
		return this.core.youtube({
			image,
			username,
			text,
		})
	}
}
