#!/bin/sh

for i in idle_0_right/????.png
do
	n=$(basename $i .png)
	n=$(echo ${n} | sed 's/^0*//')
	n=$((${n} -1))
	n=$(printf "%04d" ${n})
	o="../../../rar-content/entities/player/player-idle-right-${n}.png"
	echo ${i} "->" ${o}
	cp ${i} ${o}
done

for i in idle_0_left/????.png
do
	n=$(basename $i .png)
	n=$(echo ${n} | sed 's/^0*//')
	n=$((${n} -1))
	n=$(printf "%04d" ${n})
	o="../../../rar-content/entities/player/player-idle-left-${n}.png"
	echo ${i} "->" ${o}
	cp ${i} ${o}
done

