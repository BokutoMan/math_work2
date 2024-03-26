use my_fraction::Fraction;
const N:usize = 3;

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


/// 求d 和 l ,d 放在an的对角线上l放在下三角矩阵中
fn calculate_l_d(k:usize,result:&mut [[Fraction;N+1];N],an:&[[Fraction;N+1];N]){
    let mut c = Fraction::from_i32(0);
    c = c + an[k][k];
    for i in 0..k{
        c = c - result[k][i] * result[k][i] * result[i][i]
    }
    result[k][k] = c;
    for i in k+1..N{
        c = an[i][k];
        for j in 0..k{
            c = c - result[k][j] * result[i][j] * result[j][j]
        }
        result[i][k] = c/result[k][k]
    }
}

/// 求 Z
fn calculate_z(an:&[[Fraction;N+1];N],result:&[[Fraction;N+1];N])->[Fraction;N]{
    let mut zn = [Fraction::from_i32(0);N];
    for i in 0..N{
        let mut c = an[i][N];
        for j in 0..i{
            c = c - result[i][j] * zn[j]
        }
        zn[i] = c
    }
    zn 
}

/// 求 Y
fn calculate_y(zn:&[Fraction;N],result:&[[Fraction;N+1];N])->[Fraction;N]{
    let mut yn = [Fraction::from_i32(0);N];
    for i in 0..N{
        yn[i] = zn[i]/result[i][i]
    }
    yn 
}

/// 求x
fn calculate_x(yn:&[Fraction;N],result:&[[Fraction;N+1];N])->[Fraction;N]{
    let mut xn = [Fraction::from_i32(0);N];
    for i in (0..N).rev(){
       xn[i] = yn[i];
       for j in i+1..N {
        xn[i] = xn[i] - result[j][i] * xn[j]
       }
    }
    xn 
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


fn main(){
    let num = [
        [3,3,5,10],
        [3,5,9,16],
        [5,9,17,30]
    ];
    // 转为分数矩阵
    let xishu = get_fraction_array(num);

    // 计算d , l,d 放在an的对角线上 l 放在下三角矩阵中
    let mut result = [[Fraction::from_i32(0);N+1];N];
    for i in 0..N{
        calculate_l_d(i,&mut result,&xishu);
    }
    print_fraction_array("L-D数组为", &result);
    // LDL'X = B => L'X = Y, DY = Z, LZ = B
    //先求 Z  
    let zn = calculate_z(&xishu, &result);
    print_fraction_vector("zn = ", &zn);
    let yn = calculate_y(&zn, &result);
    print_fraction_vector("yn = ", &yn);
    let xn = calculate_x(&yn, &result);
    print_fraction_vector("xn = ", &xn);

    let val = validate(&xishu, &xn);
    print_fraction_vector("验证结果为：", &val);
    
}