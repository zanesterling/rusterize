pub fn clamp<Num: PartialOrd>(x: Num, min: Num, max: Num)
    -> Num
{
    if x < min { min }
    else if x > max { max }
    else { x }
}
