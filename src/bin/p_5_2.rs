use my_fraction::Fraction;
const N:usize = 5;

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

///得到bac矩阵
fn get_an_bn_cn(an:&[[Fraction;N+1];N])->[[Fraction;N];3]{
    let mut bca = [[Fraction::from_i32(0);N];3];
    bca[0][0] = an[0][0];
    bca[1][0] = an[0][1];
    for i in 1..N-1{
        bca[0][i] = an[i][i];
        bca[1][i] = an[i][i+1];
        bca[2][i] = an[i][i-1];
    }
    bca[0][N-1] = an[N-1][N-1];
    bca[2][N-1] = an[N-1][N-2];

    bca
}
/// 求L 与 U
fn calculate_l_u(bca:& [[Fraction;N];3])->[[Fraction;N];2]{
    let mut l_u = [[Fraction::from_i32(0);N];2];
    l_u[0][0] = bca[0][0];
    l_u[1][0] = bca[1][0]/bca[0][0];
    for i in 1..N-1{
        l_u[0][i] = bca[0][i] - bca[2][i]*l_u[1][i-1];
        l_u[1][i] = bca[1][i]/l_u[0][i];
    }
    l_u[0][N-1] = bca[0][N-1] - bca[2][N-1]*l_u[1][N-2];
    l_u
}

/// 求y
fn calculate_y(gam:& [Fraction;N],an:&[[Fraction;N+1];N],aet_bata:&[[Fraction;N];2])->[Fraction;N]{
    let mut yn = [Fraction::from_i32(0);N];
    yn[0] = an[0][N]/aet_bata[0][0];
    for i in 1..N{
        yn[i] = (an[i][N] - gam[i]*yn[i-1])/aet_bata[0][i];
    }
    yn
}

/// 求x
fn calculate_x(yn:&[Fraction;N],l_u:&[[Fraction;N];2])->[Fraction;N]{
    let mut xn = [Fraction::from_i32(0);N];
    xn[N-1] = yn[N-1];
    for i in 0..N-1{
        let i = N - 2 - i;
        xn[i] = yn[i] - l_u[1][i]*xn[i+1];
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
        [2,-1,0,0,0,1],
        [-1,2,-1,0,0,0],
        [0,-1,2,-1,0,0],
        [0,0,-1,2,-1,0],
        [0,0,0,-1,2,0]
    ];
    // 转为分数矩阵
    let xishu = get_fraction_array(num);
    // 备份一份验证时用
    let backup = xishu.clone();
    // 拿abc
    let bca = get_an_bn_cn(&xishu);
    // 计算L 和U
    let l_u = calculate_l_u(&bca);
    print_fraction_vector("L的对角元素为:", &l_u[0]);
    print_fraction_vector("U 的上三角元素为:", &l_u[1]);
    // 求解 Ly = d
    let yn = calculate_y(&bca[2], &xishu, &l_u);
    print_fraction_vector("求得的 y 为:", &yn);
    // 求解 Ux = y
    let xn = calculate_x(&yn, &l_u);
    print_fraction_vector("求得的 x 为:", &xn);

    let bn = validate(&backup,&xn);
    print_fraction_vector("验证结果为：", &bn);
}