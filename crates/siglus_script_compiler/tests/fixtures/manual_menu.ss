#z00

// Siglus manual: tutorial / scene files / _menu.ss
a[0] = sel("ゲームを始める", "ゲームを終わる")

switch (a[0]) {
    case (0) jump("５月１４日")
    case (1) owari
}

