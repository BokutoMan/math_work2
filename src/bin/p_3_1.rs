use my_fraction::Fraction;
const N:usize = 2;

/// 输出分数数组
fn print_fraction_array(name:&str,an:&[[Fraction;N+1];N]){
    println!("{name}:");
    print!("[\n");
    for i in 0..N{
        print!(" [ ");
        for j in 0..N+1{
            print!("{:?}  ",an[i][j]);
        }
        print!("]\n");
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
/// 高斯消元，k为消元的行
fn gaussian_elimination(k:usize,  an:&mut [[Fraction;N+1];N]){
    //将an(k,k)化为一
    let ankk = an[k][k];
    for i in k..N+1{
        an[k][i] = an[k][i]/ankk;
    }
    //将第k列化为0;遍历每一行
    for i in k+1..N{
        //每一行的第k列都化为0;遍历每一列
        let anik = an[i][k];
        for j in 0..N+1{
            an[i][j] = an[i][j] - an[k][j]*anik;
        }
    }
}

///验证函数
fn validate(an:[[Fraction;N+1];N],x:[[Fraction;1];N])->[[Fraction;1];N]{
    let mut bn = [[Fraction::from_i32(0);1];N];
    for i in 0..N{
        let mut zhi = Fraction::from_i32(0);
        for j in 0..N{
            zhi = zhi + an[i][j]*x[j][0];
        }
        bn[i][0] = zhi;
    }
    bn
}
/// 回代函数
fn generating(an:&mut [[Fraction;N+1];N]){
    //从最后一列开始，将每一列变为只有0和一的
    let zero = Fraction::from_i32(0);
    for j in 0..N{
        let j = N - 1 - j;
        //修改没一行的第i列和第N+1列
        for i in 0..j{
            an[i][N] = an[i][N] - an[i][j]*an[j][N];
            an[i][j] = zero;
        }
    }
}
/// 根据矩阵获取结果
fn get_result(an:[[Fraction;N+1];N])->[[Fraction;1];N]{
    let mut cn = [[Fraction::from_i32(0);1];N];
    for i in 0..N{
        cn[i][0] = an[i][N];
    }
    cn
}

fn main(){
    let num = [
        [0,1,1],
        [1,1,2]
    ];
    // 转为分数矩阵
    let mut xishu = get_fraction_array(num);
    xishu[0][0] = Fraction::new(1, 100000, true);
    // 备份一份验证时用
    let backup = xishu.clone();
    // 对每一行分别使用高斯消元
    for i in 0..N{
        gaussian_elimination(i, &mut xishu);
        print!("第{}行", i+1);
        print_fraction_array("替换后矩阵为",&xishu);
    }
    // 回代
    generating(&mut xishu);
    print_fraction_array("回代后的结果矩阵为：", &xishu);
    // 获取结果向量xn
    let xn = get_result(xishu);
    println!("xn = {:?}",xn);
    // 验证结果
    let result = validate(backup,xn);
    println!("带入计算结果得到：{:?}",result)
}