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
fn get_an_bn_cn(an:&[[Fraction;N+1];N])->[[Fraction;N];3]{
    
}
/// 追赶法求解
fn catch_up(Beta:&mut [Fraction;N], an:&mut [[Fraction;N+1];N]){
    Beta[0] = 
}

fn calculate_y()

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


fn main(){
    let num = [
        [1,2,3,-1,5],
        [2,-1,9,-7,3],
        [-3,4,-3,19,17],
        [4,-2,6,-21,-13]
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