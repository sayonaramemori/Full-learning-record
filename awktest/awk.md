### A awesome tool for text process  
1. Converge two files line by line.  
    - For a file from windows, use dos2unix to preprocess  
2. Text filter and formatter

### Basic usage 
```shell
# The default FS is <space>, $0 means the all field of the current row
# Thus this will output the all contents
awk '{print $0}' file


# You can specify the FS you want with -v 
# $num is the sequence of the field cut by FS
awk -v FS=":" '{print $1}' file
```
### Significant features of awk  
1. All variables are defalut preset. For num zero is preset, and empty string is set for string.  
2. All calculations for number is float. Use function int() to trim the dot tail.  
3. All the language styles resemble the C or C++.  

### Variable built in awk  
|Variable|Explain|Preset|  
|:--|:--|:--:|  
|FILENAME|-|-|
|FNR|Record numbers for current file|-|  
|NR|The accumulative record numbers|-|  
|NF|Numbers of fields fot current record|-|  
|FS|Field separator|" "|  
|RS|Record separator|"\n"|  
|OFS|Field separator for output|" "|  
|ORS|Record separator for output|"\n"|  


### Examples  
#### Converge two files line by line  
> For ps input  
```shell
# awk script, run it: awk -f script.awk file1 file2

BEGIN		{FS=":";OFS=",";printf("zh,en\n")}
NR==FNR		{sub(/\r$/,"");Pre[FNR]=$0;next}
			{sub(/\r$/,"");printf("%s,%s\n",Pre[FNR],$0)}
```

#### Generate content for markdown file  
```shell
# awk script below
BEGIN                           {min_hie = 6}
/^```/ && FNR==NR				{flag = flag + 1}
/^#/ && flag%2 == 0 && FNR==NR	{if(min_hie>length($1))min_hie=length($1)}
FNR==NR	                        {next}

/^```/                          {flag = flag + 1}
/^#/ && flag%2 == 0             {title=$2;anchor=tolower($2)
                                    for(i=3;i<=NF;++i){
                                        title=title " " $i
                                        anchor=anchor "-" tolower($i)
                                    }
                                    if(length($1)!=min_hie)printf("\t- [%s](#%s)\n",title,anchor)
                                        else printf("- [%s](#%s)\n",title,anchor)
                                }

# shell script to run this awk script
if [ -z $2 ];then
	name="CopyVersion_$1"
else
	name="$2"
fi
awk -f genConten.awk $1 $1 > $name
cat $1 >> $name
```




