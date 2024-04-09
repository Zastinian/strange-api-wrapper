import Core from "@hedystia/strange.core"
import Filters from "./filters"
import Generators from "./generators"
import Overlays from "./overlays"
import Utils from "./utils"

/**
 * @class
 */
export default class StrangeApi {
	core: Core
	/**
	 * @constructor
	 * @param {string} api_key
	 */
	constructor(api_key: string) {
		this.core = new Core(api_key)
	}

	/**
	 * @returns {Filters} Filters
	 */
	get filters(): Filters {
		return new Filters(this.core)
	}

	/**
	 * @returns {Generators} Generators
	 */
	get generators(): Generators {
		return new Generators(this.core)
	}

	/**
	 * @returns {Overlays} Overlays
	 */
	get overlays(): Overlays {
		return new Overlays(this.core)
	}

	/**
	 * @returns {Utils} Utils
	 */
	get utils(): Utils {
		return new Utils(this.core)
	}
}
