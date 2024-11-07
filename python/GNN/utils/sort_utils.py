def sort_dict_by_value(arg):
    kwargs = dict(arg)
    res = [(key,value) for key,value in kwargs.items()]
    res = sorted(res,key=lambda a:a[1],reverse=True)
    return res

def sort_tuple_list_by_index(arg,index):
    res = sorted(arg,key=lambda a:a[index],reverse=True)
    return res
