#!/bin/bash

target_size="1400x756"
crop_offset="+0-36"

mkdir -p ${target_size}

for i in episode-????.png
do
	# echo ${i}
	size=$(file ${i}|cut -d' ' -f5-7|cut -d',' -f1)
	# echo ${size}
	w=$(echo ${size}|cut -d' ' -f1)
	h=$(echo ${size}|cut -d' ' -f3)
	# echo ${i} ${w} ${h}
	if [[ "${target_size}" == "${w}x${h}" ]]
	then
		echo "${i} Correct size, just copy"
		cp ${i} ${target_size}/${i}
	else
		echo "${i} Wrong size, crop"
		gm convert ${i} -gravity center -crop ${target_size}${crop_offset} +repage 1400x756/${i}
	fi
done

gifski -r 1 -o ${target_size}/journey.gif ${target_size}/episode-????.png
echo done
file ${target_size}/journey.gif

cp ${target_size}/journey.gif journey.gif
