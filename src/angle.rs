use square::Square;

pub fn get_angle(to: &Square, from: &Square) -> f32
{
    let diff = (to.row as i16 - from.row as i16, to.col as i16 - from.col as i16);
    if diff.0 == 0 && diff.1 > 0
    {
        return 180.0;
    }

    else if diff.0 == 0 && diff.1 < 0
    {
        return 0.0;   
    }


    if diff.0 < 0
    {
        if diff.1 == 0
        {
            return 90.0;
        }
        else if diff.1 == -diff.0
        {
            return 135.0;
        }
        else if diff.1 == diff.0
        {
            return 45.0;
        }
    }
    else if diff.0 > 0
    {
        if diff.1 == 0
        {
            return -90.0;
        }
        else if diff.0 == -diff.1
        {
            return -45.0;
        }
        else if diff.0 == diff.1
        {
            return -135.0;
        }
    }

    if diff.0.abs() == 1
    {
        if diff.0 == -1
        {
            if diff.1 == 2
            {
                return 153.0;
            }
            else if diff.1 == -2
            {
                return 27.0;
            }
        }
        else if diff.0 == 1
        {
            if diff.1 == 2
            {
                return -153.0;
            }
            else if diff.1 == -2
            {
                return -27.0;
            }
        }

    }
    else if diff.0.abs() == 2
    {
        if diff.0 == -2
        {
            if diff.1 == 1
            {
                return 116.5;
            }
            else if diff.1 == -1
            {
                return 63.5;
            }
        }
        else if diff.0 == 2
        {
            if diff.1 == 1
            {
                return -116.5;
            }
            else if diff.1 == -1
            {
                return -63.5;
            }
        }
    }


    0.0
}
