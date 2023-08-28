let w, h = @Dimensions()

text = @truncate(text, 28, "...")

@Clear(#1b1b1b)

@SetFont("discord-bold noto-jp notob")
@SetFontSize(40)
@DrawStringAnchored(text, w / 2, h / 2, 0.5, 0.5)

// @DrawTextBoxWrapped(text, w / 2, h / 2, 0.5, 0.5, w - 20, h, "left")
@SetColor(#ffffff)
@Fill()