BEGIN		{FS=":";OFS=","
				printf("zh,en\n");
			}
NR==FNR		{sub(/\r$/,"");Pre[FNR]=$0;next}
			{sub(/\r$/,"");printf("%s,%s\n",Pre[FNR],$0)}
