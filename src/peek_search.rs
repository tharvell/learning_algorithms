
pub fn binomial_peek_search(data: &[i16]) -> i16{
   let mid: usize = data.len()/2;
   
   if data.len() == 1 {
      return data[mid];
   }else if data.len() == 2{
      if data[0] >= data[1]{
          return data[0];
      } else {
          return data[1]
      }
   } 
   else if data[mid-1] >= data[mid]{
      return binomial_peek_search(&data[..mid]);
   }else if data[mid+1] >= data[mid]{
      return binomial_peek_search(&data[mid..]);
   }else{
      return data[mid]; 
   }
}

pub fn linear_peek_search(data: &Vec<i16>) -> i16{
    for (i, _x) in data.iter().enumerate()  {
    	if i==0{
           if data[0] > data[1]{
               return data[0];
           }
        }else if i==data.len()-1{
	   if data[i] > data[i-1]{
               return data[i];
           }  
        }else if data[i-1] <= data[i] && data[i+1] <= data[i]{
           return data[i];
        }
    }
    return -1;
}	


