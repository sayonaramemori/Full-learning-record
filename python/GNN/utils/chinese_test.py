def test_chinese():
    import matplotlib
    import matplotlib.pyplot as plt
    matplotlib.rc("font",family='SimHei')
    plt.rcParams['axes.unicode.minus'] = False
    plt.plot([1,2,3],[100,300,200])
    plt.title('matplotlib 中文字测试',fontsize=25)
    plt.xlabel('X轴',fontsize=15)
    plt.ylabel('Y轴',fontsize=15)
    plt.show()

