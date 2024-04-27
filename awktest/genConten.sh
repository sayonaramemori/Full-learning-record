if [ -z $2 ];then
	name="CopyVersion_$1"
else
	name="$2"
fi
awk -f genConten.awk $1 $1 > $name
cat $1 >> $name


