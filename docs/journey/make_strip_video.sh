#!/bin/bash

declare -i target_w=1400
declare -i target_h=756
declare -i gap_x=100
declare -i speed=60
declare -i fps=60
declare -i pixels_per_frame=4

target_size="${target_w}x${target_h}"

# Note: this assumes make_anim already prepped

# make a very long side by side

declare -i count=$(ls ${target_size}/episode-*.png|wc -l)

echo ${count}


declare -i gaps=count-1
declare -i width=target_w*count+gap_x*gaps
declare -i height=target_h

echo ${target_w}  ${count}

echo ${width} x ${height}

#gm montage ${target_size}/episode-*.png  -tile ${count}x1 -geometry ${target_w}x${target_h}\>+1+1  ${target_size}/strip.png
#gm montage ${target_size}/episode-*.png  -tile x1 -geometry ${target_w}x${target_h}\>+1+1  ${target_size}/strip.png
#gm montage ${target_size}/episode-*.png  -tile 999x1  -geometry ${target_w}x${target_h}\>+${gap_x}+0 ${target_size}/strip.png
geom="43400x347+${gap_x}+100"
echo ${geom}
#gm montage ${target_size}/episode-*.png  -tile 999x9  -geometry ${geom} ${target_size}/strip.png
#gm montage ${target_size}/episode-*.png  -tile x1  -geometry +100+0 ${target_size}/strip.png
#magick montage ${target_size}/episode-*.png  -tile x1  -geometry +100+0 -background none ${target_size}/strip.png
file ${target_size}/strip.png

size=$(file ${target_size}/strip.png|cut -d' ' -f5-7|cut -d',' -f1)
# echo ${size}
declare -i w=$(echo ${size}|cut -d' ' -f1)
declare -i h=$(echo ${size}|cut -d' ' -f3)


echo "Rendering strip frames"
declare -i steps=w/4+2*1920
declare -i seconds=steps
echo ${seconds}
seconds=seconds*10000
echo ${seconds}
seconds=seconds/pixels_per_frame
echo ${seconds} : pixels_per_frame
seconds=seconds/fps
echo ${seconds} : fps
#seconds=seconds*10000
#echo ${seconds}
#seconds=seconds*10/27
#echo ${seconds} : /27
#seconds=seconds/1000
#echo ${seconds}
seconds=seconds/10000
echo ${seconds}

seconds=seconds


ffmpeg -y \
	-f lavfi -i "\
		color=0x00000000:r=${fps}:d=${seconds}:s=1920x1080[background]; \
		movie=${target_size}/strip.png[overlay]; \
		[overlay]scale=iw/4:-1[overlay]; \
		[background][overlay]overlay='W-n*${pixels_per_frame}:130' \
	" \
    -c:v libx264 -x264-params lossless=1 \
    -pix_fmt yuv420p \
    ${target_size}/journey.mp4

#ffmpeg -y \
#	-f lavfi -i "\
#		color=0x00000000:r=60:d=66:s=1920x1080[background]; \
#		movie=${target_size}/strip.png[overlay]; \
#		[overlay]scale=-2:260[overlay]; \
#		[background][overlay]overlay='W-n*4:(H-h)/4' \
#	" \
#    -c:v libx264 -x264-params lossless=1 \
#    -pix_fmt yuv420p \
#    ${target_size}/journey.mp4

#    -i ${target_size}/strip.png -filter_complex '[background][overlay]overlay=10:main_h-overlay_h-10'\
#	-f lavfi -i color=size=1920x1080:duration=15:rate=60:color=0x00000000[background]; \
#	-i ${target_size}/blank.png \
#   -pix_fmt yuv420p -vf "scale=1920:1080,loop=-1:1" \
 #   -movflags faststart \
#    -vf "scale=1920:1080" \
 #   ${target_size}/journey.mp4

#    -pix_fmt yuv420p -vf "scale=1920:1080,loop=-1:1" \

#ffmpeg -y -framerate 60 -i ${target_size}/blank.png -t 15 \
#    -c:v libx265 -x265-params lossless=1 \
#    -pix_fmt yuv420p -vf "scale=1920:1080,loop=-1:1" \
#    -movflags faststart \
#    ${target_size}/journey.mp4

ffprobe ${target_size}/journey.mp4