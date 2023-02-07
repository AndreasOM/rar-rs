use framework "Foundation"
use framework "AppKit"
use scripting additions

on run argv 

	set theApp to "rar"
	set xPos to 0
	set yPos to 0
	set width to 1400 / 2 as integer
	set height to 756 /2 as integer

	tell application "System Events" to tell process theApp
		tell window 1
			set position to {xPos, yPos}
			set size to {width, height}
		end tell
	end tell
end run

