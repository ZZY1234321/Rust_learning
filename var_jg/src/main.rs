// struct: 定义一个结构体，包含一个字段 e（i32 类型）
// main 函数：声明并初始化变量，并使用解构赋值来赋值变量
// 解构赋值：将元组或结构体的多个值分别赋值给多个变量
// 元组解构赋值：将元组的多个值分别赋值给多个变量
struct Struct {
    e: i32
}

fn main() {
    // 声明变量 a, b, c, d, e，但是没有赋值
    let (a, b, c, d, e);
    // 使用解构赋值来赋值变量
    // 将元组 (1, 2) 中的值分别赋给变量 a 和 b，现在 a 的值为 1，b 的值为 2。
    (a, b) = (1, 2);
    // _ 代表匹配一个值，但是我们不关心具体的值是什么，因此没有使用一个变量名而是使用了 _
    // [c,.., d, _] 是一种特殊的模式，c 会匹配数组的第一个元素，所以 c 会被赋值为 1；d 会匹配数组倒数第二个元素，所以 d 被赋值为 4；_ 表示匹配一个值但我们不关心它具体是什么，在这里就是忽略了数组中的最后一个元素 5。
    [c, .., d, _] = [1, 2, 3, 4, 5];
    // Struct { e,.. } 这种模式表示提取结构体 Struct 中的 e 字段进行匹配，在这里会将右侧 Struct { e: 5 } 中的 e 字段的值（也就是 5）赋给变量 e，同时忽略结构体中其他可能的字段（这里只有一个字段所以不存在其他忽略情况）。
    Struct { e, .. } = Struct { e: 5 };

    // 使用 assert_eq! 宏来断言两个数组 [1, 2, 1, 4, 5] 和 [a, b, c, d, e] 的元素是否完全相等，如果相等则程序继续正常执行，如果不相等则会触发 panic（导致程序异常终止并输出相关错误信息），用于验证前面赋值操作后各个变量的值是否符合预期。

    // 现在a是1，b是2，c是1，d是4，e是5，所以下面这行代码不会触发 panic。
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}