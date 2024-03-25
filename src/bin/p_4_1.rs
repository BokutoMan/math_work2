use my_fraction::Fraction;
const N:usize = 4;

/// 输出分数数组
fn print_fraction_array(name:&str,an:&[[Fraction;N+1];N]){
    println!("{name}:");
    print!("[\n");
    for i in 0..N{
        print!(" [ ");
        for j in 0..N+1{
            if an[i][j].is_int(){
                print!("{}  ", an[i][j].get_int())
            }else{
                print!("{:?}  ",an[i][j]);
            }
        }
        print!("]\n");
    }
    print!("]\n");
}
fn print_fraction_vector(name:&str,an:&[Fraction;N]){
    println!("{name}");
    print!(" [ ");
        for j in 0..N{
            if an[j].is_int(){
                print!("{}  ", an[j].get_int())
            }else{
                print!("{:?}  ",an[j]);
            }
        }
        print!("]\n");
}
/// 将整数数组转换为分数数组
fn get_fraction_array(an:[[i32;N+1];N])->[[Fraction;N+1];N]{
    let mut kn:[[Fraction;N+1];N] = [[Fraction::from_i32(0);N+1];N];
    for i in 0..N{
        for j in 0..N+1{
            kn[i][j] = Fraction::from_i32(an[i][j]);
        }
    }
    kn
}
/// 直接分解法,k 为当前分解的U的行和L的列
fn direct_decomposition(k:usize,  an:&mut [[Fraction;N+1];N]){
    // 计算第一行和第一列
    if k == 0 {
        for i in 1..N{
            an[i][0] = an[i][0]/an[0][0];
        }
        return
    }
    // 计算第k行元素
    for j in k..N{
        let mut c = Fraction::from_i32(0);
        for i in 0..k{
            c = c + an[k][i]*an[i][j];
        }
        an[k][j] = an[k][j] - c;
    }
    // 计算第 k 列元素
    for i in k+1..N{
        let mut c = Fraction::from_i32(0);
        for j in 0..k{
            c = c + an[i][j]*an[j][k];
        }
        an[i][k] = (an[i][k] - c)/an[k][k];
    }
}

///根据聚合矩阵得到L 和 U 
fn get_l_and_u(ln:&mut [[Fraction;N+1];N],un:&mut [[Fraction;N+1];N],an:&[[Fraction;N+1];N]){
    for i in 0..N{
        ln[i][i] = Fraction::from_i32(1);
        for j in 0..i{
            ln[i][j] = an[i][j]
        }
        for j in i..N{
            un[i][j] = an[i][j]
        }
        ln[i][N] = an[i][N];
        un[i][N] = an[i][N];
    }
}

///验证函数
fn validate(an:& [[Fraction;N+1];N],x:&[Fraction;N])->[Fraction;N]{
    let mut bn = [Fraction::from_i32(0);N];
    for i in 0..N{
        let mut zhi = Fraction::from_i32(0);
        for j in 0..N{
            zhi = zhi + an[i][j]*x[j];
        }
        bn[i] = zhi;
    }
    bn
}
/// 计算y
fn calculate_y(ln:& [[Fraction;N+1];N])->[Fraction;N]{
    let mut yn = [Fraction::from_i32(0);N];
    yn[0] = ln[0][N];
    // 计算剩下的y值
    for i in 1..N{
        let mut c = Fraction::from_i32(0);
        for j in 0..i{
            c = c + ln[i][j]*yn[j];
        }
        yn[i] = ln[i][N] - c;
    }
    yn
}
/// 计算x
fn calculate_x(un:& [[Fraction;N+1];N], yn:& [Fraction;N])->[Fraction;N]{
    let mut xn = [Fraction::from_i32(0);N];
    xn[N-1] = yn[N-1]/un[N-1][N-1];
    // 计算剩下的x
    for i in 0..N-1{
        let i = N - 2- i;
        let mut c = Fraction::from_i32(0);
        for j in i+1..N{
            c = c + un[i][j]*xn[j];
        }
        xn[i] = (yn[i] - c)/un[i][i];
    }
    xn
}

fn main(){
    let num = [
        [1,0,2,0,5],
        [0,1,0,1,3],
        [1,2,4,3,17],
        [0,1,0,3,7]
    ];
    // 转为分数矩阵
    let mut xishu = get_fraction_array(num);
    // 备份一份验证时用
    let backup = xishu.clone();
    // 对每一行和列分别进行分解
    for i in 0..N{
        direct_decomposition(i, &mut xishu)
    }

    print_fraction_array("分解得到的聚合矩阵为",&xishu);
    let mut ln = [[Fraction::from_i32(0);N+1];N];
    let mut un = [[Fraction::from_i32(0);N+1];N];
    get_l_and_u(&mut ln, &mut un, &xishu);
    let yn = calculate_y(&mut ln);
    let xn = calculate_x(&un, &yn);
    print_fraction_array("L 为", &ln);
    print_fraction_array("U 为", &un);
    print_fraction_vector("yn = ", &yn);
    print_fraction_vector("xn = ", &xn);
    let bn = validate(&backup,&xn);
    print_fraction_vector("验证结果为：", &bn);
}