# Playdate font converter

Generates [Playdate font
files](https://sdk.play.date/3.0.1/Inside%20Playdate.html#C-graphics.font) from
a local font file and a size.

Example:

```
$ cargo run /usr/share/fonts/noto/NotoSans-Regular.ttf 20
Wrote NotoSans-Regular-20-table-19-28.png
Wrote NotoSans-Regular-20.fnt
```

To load the font in your Playdate game:

```
local font, err = playdate.graphics.font.new('NotoSans-Regular-20')

if err then
    error(err)
end
```
