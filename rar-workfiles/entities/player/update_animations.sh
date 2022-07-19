#!/bin/sh

copy_action() {
	prefix=$1
	suffix=$2
	first_frame=$3
	last_frame=$4
	output=$5

	echo $1 $2 $3 $4 $5
	for i in ${prefix}/????${suffix}.png
	do
		# echo $i
		n=$(basename $i ${suffix}.png)
		if [ ${n} == "0000" ]
		then
			n=0
		else
			n=$(echo ${n} | sed 's/^0*//')
		fi
		# echo "? ${first_frame} <= ${n} <= ${last_frame}"
		if [ ${n} -ge ${first_frame} ] && [ ${n} -le ${last_frame} ]
		then
			n=$((${n} -${first_frame}))
			n=$(printf "%04d" ${n})
			# echo ${n}
			o="../../../rar-content/entities/player/${output}-${n}.png"
			echo ${i} "->" ${o}
			cp ${i} ${o}
		fi
		# echo ${n}
	done

}

copy_action "combined" "_ToRight" 0 8 player-idle-right
copy_action "combined" "_ToLeft" 0 8 player-idle-left

copy_action "combined" "_ToRight" 9 57 player-backflip-right
copy_action "combined" "_ToLeft" 9 57 player-backflip-left
