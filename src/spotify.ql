// let track_name = "Chamber Of Reflection"
// let artist_name = "Mac DeMarco"
// let progress = 0.5
// let time = "2:16"

let bg = @GetCoverColor(art)

let bg_end = bg - @rgba(bg[0] / 2, bg[1] / 2, bg[2] / 2, 0)
let w, h = @Dimensions()

@DrawRoundedRectangle(0, 0, w, h, 10)
@SetLinearGradient((0, 0), (w, 0), "pad", [(0.0, bg), (0.6, bg_end)])
@Fill()

let pad = 20

@SetFilter("fast")

@DrawRoundedRectangle( pad / 2, pad / 2, h - pad, h - pad, 8)
@Clip()

@DrawImageSized(art, pad / 2, pad / 2, h - pad, h - pad)
@ResetClip()

let track_name = @truncate(track_name, 24, "...")
let artist_name = @truncate(artist_name, 34, "...")
let progress = @min(progress, 1.0)

@SetColor(#ffffff)

@SetFont("discord-bold notojp")
@SetFontSize(30)
@DrawString(track_name, h, 15)
@Fill()

@SetFont("discord-semibold notojp")
@SetFontSize(18)
@DrawString(artist_name, h, 42)
@Fill()


let mw = w - h - pad
@DrawRoundedRectangle(h, 72, mw, 8, 10)
@SetColor(#0000003e)
@Fill()

@DrawRoundedRectangle(h, 72, mw * progress, 8, 10)
@SetColor(#ffffff)
@Fill()

@DrawStringAnchored(time, w - pad - 2, 53, 1.0, 0.0)
@Fill()
