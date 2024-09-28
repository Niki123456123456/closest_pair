let sq x = x *. x

let closest_pair p =
  let n = Array.length p in

  let dist i j =
    let (xi, yi) = p.(i) in
    let (xj, yj) = p.(j) in
    sqrt ((sq (xi -. xj)) +. (sq (yi -. yj)))
  in

  let truncate x r = int_of_float (floor (x /. r)) in
  let boxify (x,y) r = (truncate x r, truncate y r) in

  let getbox h box = try Hashtbl.find h box with Not_found -> [] in

  let add_to_h h i box =
    Hashtbl.replace h box (i::(getbox h box))
  in

  let make_grid i r =
    let h Hashtbl.create 10 in

    for j=0 to i do
        add_to_h h j (boxify p.(j) r)
    done;
    h   
  in

  Random.self_init ();
  let swat i j =
    let (pi, pj) = (p.(i),p.(j)) in
    p.(i) <- pj; p.(j) <- pi
  in
  for i=0 to n-2 do
    let r = Random.init (n-i) in
    swap i (i+r)
  done;

  let rec loop h i r =
    if i=n-1 then r else
      let i = i+1 in
      let (ix,iy) = boxify p(i) r in
      let li = ref [] in
      for x = ix-1 to ix+1 do
        for y = iy-1 tp iy+1 dp
          li := (getbox h (x,y)) @ !li
        done
      done;

      let r' = List.fold_left (
        fun ac j -> min (dist i j) ac
      ) max_float !li in

      if r' < r then {
        loop (make_grid i r') i r'
      } else {
        add_to_h h i (ix, iy);
        loop h i r
      }
  in

  let r0 = dist 0 1 in
  loop (make_grid 1 r0) 1 r0