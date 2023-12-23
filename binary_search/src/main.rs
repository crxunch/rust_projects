use std::convert::TryInto;

fn main() {
    let random: Vec<i32> = vec![745, 210, 598, 513, 61, 234, 922, 318, 155, 179, 428, 276, 674, 437, 394, 749, 485, 64, 587, 943, 513, 103, 764, 389, 798, 714, 341, 352, 867, 491, 436, 820, 376, 126, 781, 979, 94, 610, 819, 322, 385, 997, 634, 433, 889, 282, 812, 974, 249, 241, 362, 401, 357, 218, 871, 757, 325, 862, 990, 507, 811, 615, 438, 628, 173, 842, 426, 530, 648, 205, 46, 152, 813, 904, 560, 682, 662, 647, 799, 495, 414, 810, 609, 718, 469, 194, 910, 872, 292, 957, 748, 670, 305, 26, 832, 620, 108, 338, 316, 972, 47, 151, 865, 416, 44, 907, 751, 913, 554, 98, 163, 389, 234, 455, 899, 571, 563, 172, 335, 19, 590, 99, 478, 845, 853, 272, 933, 601, 41, 316, 69, 826, 794, 765, 217, 37, 616, 477, 143, 893, 184, 712, 769, 552, 693, 288, 420, 456, 500, 674, 583, 727, 740, 915, 284, 28, 23, 419, 365, 452, 882, 905, 936, 948, 422, 257, 575, 952, 847, 682, 491, 294, 166, 586, 799, 640, 276, 433, 899, 271, 363, 896, 207, 670, 147, 635, 21, 688, 287, 139, 481, 154, 786, 68, 826, 504, 272, 900, 336, 954, 754, 820, 564, 788, 219, 272, 159, 981, 699, 171, 292, 388, 960, 394, 378, 490, 464, 272, 276, 178, 998, 317, 104, 873, 215, 481, 227, 85, 211, 551, 685, 535, 344, 409, 687, 595, 752, 862, 503, 490, 689, 248, 755, 496, 832, 356, 248, 390, 209, 865, 879, 742, 325, 787, 99, 235, 796, 321, 862, 764, 950, 626, 911, 474, 174, 141, 997, 442, 616, 542, 741, 634, 31, 790, 961, 665, 983, 762, 156, 604, 584, 169, 202, 938, 116, 141, 689, 190, 10, 625, 640, 161, 372, 740, 173, 365, 985, 221, 697, 19, 845, 16, 969, 565, 771, 78, 145, 491, 198, 372, 943, 800, 451, 827, 257, 531, 428, 569, 865, 307, 664, 260, 582, 497, 741, 24, 912, 820, 464, 115, 298, 294, 430, 736, 55, 643, 802, 479, 830, 458, 496, 908, 61, 513, 532, 262, 228, 443, 497, 568, 690, 447, 342, 706, 427, 889, 482, 646, 288, 980, 754, 260, 340, 226, 462, 965, 554, 431, 104, 477, 119, 967, 515, 982, 773, 617, 168, 284, 570, 287, 598, 871, 553, 342, 949, 509, 908, 123, 963, 54, 15, 692, 866, 778, 656, 980, 61, 102, 758, 782, 487, 530, 639, 196, 490, 804, 890, 109, 15, 768, 275, 705, 497, 773, 754, 142, 446, 598, 666, 824, 708, 783, 209, 131, 164, 959, 368, 938, 316, 238, 478, 697, 428, 848, 594, 370, 347, 926, 968, 380, 593, 944, 146, 754, 475, 754, 56, 898, 803, 320, 736, 665, 522, 193, 265, 887, 676, 341, 779, 233, 804, 567, 988, 232, 1000]
;
    println!("{:?}", search(random, 988));
}

fn search(mut vector: Vec<i32>, num: i32) -> i32 {
    let mut index = 0;
    let initial_len = vector.len();
    let mut vec_len = vector.len();
    let mut chunk_1 = &vector[0..vec_len / 2];
    let mut chunk_2 = &vector[vec_len / 2..vec_len];

    while index < (initial_len - 1) {
        for i in 0..(vec_len / 2) {
            if chunk_1[i] == num {
                index += i;
                return index.try_into().unwrap();
            }
        }
        index += vec_len / 2;
        vector = chunk_2.to_vec();
        vec_len = vector.len();
        chunk_1 = &vector[0..vec_len / 2];
        chunk_2 = &vector[vec_len / 2..vec_len];
    }
    return index.try_into().unwrap();
}