BEGIN							{min_hie = 6}
/^```/ && FNR==NR				{flag = flag + 1}
/^#/ && flag%2 == 0 && FNR==NR	{if(max_hie<length($1))max_hie=length($1);
								 if(min_hie>length($1))min_hie=length($1);
								}
FNR==NR							{next}

/^```/				{flag = flag + 1}
/^#/ && flag%2 == 0 {title=$2;
					 anchor=tolower($2);
					 for(i=3;i<=NF;++i){
						 title=title " " $i;
						 anchor=anchor "-" tolower($i);
					 }
					 if(length($1)!=min_hie)printf("\t- [%s](#%s)\n",title,anchor);
					 else printf("- [%s](#%s)\n",title,anchor);
					}
