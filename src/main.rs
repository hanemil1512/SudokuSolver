/*
**Það sem þarf aða gera**
1. passa að hver reitur er ekki með sömu töluna
2. Passa að hver lína er ekki með sömu töluna
3. passa að hver kassi er ekki með sömu töluna
*/

fn is_vec_unique(v: &Vec<i8>) -> bool{
    let mut xcon = 0;
    let mut ycon;
    for numx in v{
        ycon = 0;
        for numy in v{
            if xcon == ycon{
                ycon += 1;
                continue;
            }else if numy == numx{
 
                return false;
            }
            ycon += 1;
        }
        xcon += 1;
    }
    return true;
}

fn remove_vec_replicas(v: &Vec<i8>){
    let mut xcon = 0;
    let mut ycon;
    for numx in v{
        ycon = 0;
        for numy in v{
            if xcon == ycon{
                ycon += 1;
                continue;
            }else if numy == numx{
 
                return false;
            }
            ycon += 1;
        }
        xcon += 1;
    }
    return true;
}
/*
 * Plannið fyrr alla níu kassana var að gera formúlu fyrir kassa 1
 * en svo myndi maður bara plúsa um 3 fyrir hvern reit hægra megin
 * og 3 fyrir hvern reit vinstra megin þannig maður myndi enda með
 * +6 á x hlið og + 3 á y hlið á kassanum númer 6. Númera skipulag
 * er hægt að sjá að neðan:
______
|1|2|3|
|4|5|6|
|7|8|9|
‾‾‾‾‾‾
 * */
fn is_solvable(board: &Vec<Vec<i8>>) -> bool{
    let mut vec_x: Vec<i8> = Vec::new();
    let mut vec_y: Vec<i8> = Vec::new();
    let mut vec_box: Vec<i8> = Vec::new(); 
    //i8 svo að það er sama týpa þegar er reiknað með box_y og box_x
    for x in 0i8..9{
        for y in 0i8..9{
            //þarf að nota usize í vector þannig það er síðan breytt til baka 
            if board[x as usize][y as usize] != 0 {
                vec_x.push(board[x as usize][y as usize]);
            }
            if board[y as usize][x as usize] != 0 {
                vec_y.push(board[y as usize][x as usize]); 
            }
            //Holy shit hvað þetta er sniðugt!!
            //þetta passar að boxX er alltaf á bilinu 0-2
            //
            let box_x: i8 = y + 3*(x % 3 - y/3);
            let box_y: i8 = y/3 + 3*(x/3);
            if board[box_x as usize][box_y as usize] != 0 {
                vec_box.push(board[box_x as usize][box_y as usize])
            }
        }
        if (is_vec_unique(&vec_x) == false) || (is_vec_unique(&vec_y) == false) || (is_vec_unique(&vec_box) == false){
            return false;
        }
        vec_x.clear();
        vec_y.clear();
        vec_box.clear(); 
   }
    return true;
}


fn solve_sudoku_board(solved_board: &Vec<Vec<i8>>){
    let mut square: Vec<Vec<i8>> = vec![vec![0;9];9];
    loop{
        for x in 0i8..9{
            //Fluff þarf að vera resetað eftir hvern kasss
            let mut fluff: Vec<Vec<i8>> = vec![vec![1,2,3,4,5,6,7,8,9];9];
            for y in 0i8..9{
                //Formúla sem lætur skoða einn kassa í einu, skoða efri mynd
                let box_x: i8 = y + 3*(x % 3 - y/3);
                let box_y: i8 = y/3 + 3*(x/3);

            }
        }
    }

}

fn main() {
    //board[row][col]
    let unsolved_board: Vec<Vec<i8>> = vec![
    vec![5, 3, 0, 0, 7, 0, 0, 0, 0],
    vec![6, 0, 0, 1, 9, 5, 0, 0, 0],
    vec![0, 9, 8, 0, 0, 0, 0, 6, 0],
    vec![8, 0, 0, 0, 6, 0, 0, 0, 3],
    vec![4, 0, 0, 8, 0, 3, 0, 0, 1],
    vec![7, 0, 0, 0, 2, 0, 0, 0, 6],
    vec![0, 6, 0, 0, 0, 0, 2, 8, 0],
    vec![0, 0, 0, 4, 1, 9, 0, 0, 5],
    vec![0, 0, 0, 0, 8, 0, 0, 7, 9],
    ];
    
    match is_solvable(&unsolved_board){
        true => println!("This sudoku board is solvable!"),
        false => println!("This sudoku board is NOT solvable!"),
    }
    //println!("this is a test");
    //println!("{:#?}", _board);
}
