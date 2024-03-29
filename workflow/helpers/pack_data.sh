#!/bin/bash

argc=$#
if [ $argc -lt 3 ]
then
	echo "Usage: $0 [basepath] [output.omar] [paklist] {music_ogg|music_mp3|music_mp3_ogg}"
	exit -1
fi
basepath=$1
shift 1

output=$1
shift 1

output_omna="${output}.omna"

paklist=$1
shift 1

music=""

#data_prefix=${output%.omar}
data_prefix=$(basename ${output} .omar)

while [ "$1" != "" ]
do
	echo $1
	case $1 in
		"music_ogg")
			music="ogg"
			;;
		"music_mp3")
			music="mp3"
			;;
		"music_mp3_ogg")
			music="mp3_ogg"
			;;
		"music_ogg_mp3")
			music="mp3_ogg"
			;;
		*)
			echo "Unknown parameter: $1"
			exit -1
			;;
	esac

	shift 1
done

echo "basepath:      ${basepath}"
echo "output:        ${output}"
echo "data_prefix:   ${data_prefix}"
echo "paklist:       ${paklist}"
echo "music:         ${music}"

case ${music} in
	"ogg")
		exclude_music=".mp3"
		;;
	"mp3")
		exclude_music=".ogg"
		;;
	"mp3_ogg")
		exclude_music=".nothing_to_exclude"
		;;
	*)
		echo "Music format not selected"
		exit -1
		;;
esac

echo "music to exclude: ${exclude_music}"

bn_file=$(dirname "$0")/../../build_number.txt

cp ${bn_file} ${basepath}/${data_prefix}_build_number.txt
# :TODO: have variant specific paklist and keep it

cd ${basepath}
ls -1 |grep -v ${paklist} |grep -v "${exclude_music}" >${paklist}
cd -
echo omt-packer pack \
	--basepath ${basepath} \
	--output ${output} \
	--paklist ${basepath}/${paklist} \
	--name-map ${output_omna}

omt-packer pack \
	--basepath ${basepath} \
	--output ${output} \
	--paklist ${basepath}/${paklist} \
	--name-map ${output_omna}

