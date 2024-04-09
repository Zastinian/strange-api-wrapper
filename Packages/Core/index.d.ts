declare class StrangeApi {
	constructor(api_key: string)
	blur(data: { image: string; level: number }): ArrayBuffer
	brighten(data: { image: string; amount: number }): ArrayBuffer
	burn(data: { image: string; level: number }): ArrayBuffer
	darken(data: { image: string; amount: number }): ArrayBuffer
	deepfry(data: { image: string }): ArrayBuffer
	distort(data: { image: string; level: number }): ArrayBuffer
	greyscale(data: { image: string }): ArrayBuffer
	invert(data: { image: string }): ArrayBuffer
	pixelate(data: { image: string; pixels: number }): ArrayBuffer
	sepia(data: { image: string }): ArrayBuffer
	sharpen(data: { image: string; level: number }): ArrayBuffer
	threshold(data: { image: string; amount: number }): ArrayBuffer
	pokemon3000Years(data: { image: string }): ArrayBuffer
	ad(data: { image: string }): ArrayBuffer
	affect(data: { image: string }): ArrayBuffer
	batslap(data: { image1: string; image2: string }): ArrayBuffer
	beautiful(data: { image: string }): ArrayBuffer
	bed(data: { image1: string; image2: string }): ArrayBuffer
	bobross(data: { image: string }): ArrayBuffer
	challenger(data: { image: string; silhouetted: boolean }): ArrayBuffer
	changemymind(data: { text: string }): ArrayBuffer
	clown(data: { image: string }): ArrayBuffer
	clyde(data: { text: string }): ArrayBuffer
	confusedstonk(data: { image: string }): ArrayBuffer
	delete(data: { image: string }): ArrayBuffer
	demotivational(data: { title: string; text: string; image: string }): ArrayBuffer
	dexter(data: { image: string }): ArrayBuffer
	discordblack(data: { image: string }): ArrayBuffer
	discordblue(data: { image: string }): ArrayBuffer
	doublestonk(data: { image1: string; image2: string }): ArrayBuffer
	facepalm(data: { image: string }): ArrayBuffer
	fusion(data: { image1: string; image2: string }): ArrayBuffer
	gruPlan(data: {
		first_step: string
		second_step: string
		third_step: string
		fourth_step: string
	}): ArrayBuffer
	heartbreaking(data: { image: string }): ArrayBuffer
	hitler(data: { image: string }): ArrayBuffer
	jail(data: { image: string }): ArrayBuffer
	jokeoverhead(data: { image: string }): ArrayBuffer
	karaba(data: { image: string }): ArrayBuffer
	kiss(data: { image1: string; image2: string }): ArrayBuffer
	kyonGun(data: { image: string }): ArrayBuffer
	lisaPresentation(data: { text: string }): ArrayBuffer
	mikkelsen(data: { image: string }): ArrayBuffer
	mirror(data: { image: string }): ArrayBuffer
	mms(data: { image: string }): ArrayBuffer
	notstonk(data: { image: string }): ArrayBuffer
	ohno(data: { text: string }): ArrayBuffer
	planktonPlan(data: {
		first_step: string
		second_step: string
		third_step: string
		fourth_step: string
	}): ArrayBuffer
	poutine(data: { image: string }): ArrayBuffer
	rip(data: { image: string }): ArrayBuffer
	shit(data: { image: string }): ArrayBuffer
	snyder(data: { image: string }): ArrayBuffer
	spank(data: { image1: string; image2: string }): ArrayBuffer
	stonk(data: { image: string }): ArrayBuffer
	tattoo(data: { image: string }): ArrayBuffer
	thomas(data: { image: string }): ArrayBuffer
	trash(data: { image: string }): ArrayBuffer
	wanted(data: { image: string }): ArrayBuffer
	worthless(data: { image: string }): ArrayBuffer
	youtube(data: { image: string; username: string; text: string }): ArrayBuffer
	approved(data: { image: string }): ArrayBuffer
	brazzers(data: { image: string }): ArrayBuffer
	gay(data: { image: string }): ArrayBuffer
	halloween(data: { image: string }): ArrayBuffer
	rejected(data: { image: string }): ArrayBuffer
	thuglife(data: { image: string }): ArrayBuffer
	toBeContinued(data: { image: string }): ArrayBuffer
	wasted(data: { image: string }): ArrayBuffer
	circle(data: { image: string }): ArrayBuffer
	color(data: { code: string }): ArrayBuffer
	denoise(data: { image: string }): ArrayBuffer
	farewellCard(data: {
		avatar: string
		name: string
		discriminator: string
		count: number
		guild: string
		bkg: string
	}): ArrayBuffer
	rankCard(data: {
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
	}): ArrayBuffer
	spotifyCard(data: {
		image: string
		author: string
		album: string
		start: number
		end: number
		title: string
	}): ArrayBuffer
	welcomeCard(data: {
		avatar: string
		name: string
		discriminator: string
		count: number
		guild: string
		bkg: string
	}): ArrayBuffer
}

export default StrangeApi

declare module "./index.node" {
	export { StrangeApi }
}
