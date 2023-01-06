use std::io::Write;
use std::fs::OpenOptions;
use std::io::Read;


fn code_7(){
 let mut file = std::fs::File::create("aigbon_juillet.txt").expect("creat failed");
 file.write_all("Dept:Consulting\nQualification:B.Sc.\nCode:7".as_bytes()).expect("write failed");

 let mut file = std::fs::File::create("akpevwe_iloka.txt").expect("creat failed");
 file.write_all("Dept:Assurance\nQualification:HND\nCode:7".as_bytes()).expect("write failed");
 
}
fn code_8(){
    let mut file = std::fs::File::create("adamu_sagamu.txt").expect("creat failed");
 file.write_all("Dept:Tax\nQualification:B.Sc.\nCode:8".as_bytes()).expect("write failed");
 
 let mut file = std::fs::File::create("gbenga_daniels.txt").expect("creat failed");
 file.write_all("Dept:People and Workforce\nQualifications:HND\nCode:8".as_bytes()).expect("write failed");
    
}
fn code_9(){
  let mut file = std::fs::File::create("ehis_ero.txt").expect("create failed");
 file.write_all("Dept:Strategy\nQualification:M.Sc.\nCode:9".as_bytes()).expect("write failed");
  
 let mut file = std::fs::File::create("maria_akinsola.txt").expect("create failed");
 file.write_all("Dept:Transactions and corporate finace\nQualification:M.Sc.\nCode:9".as_bytes()).expect("write failed");    
}
use std::io;

fn main() {

    println!("C for Consulting , A for Assurance, T for Tax, P for People and Workforce, S for Strategy, N for Transactions and corporate finace");
    let mut input2 = String::new;
    let mut input1 = String::new;

   println!("Enter code: ");
    io::stdin().read_line(&mut input1).expect("Invalid input"); 
    let code:i32 = input1.trim().parse().expect("Invalid order");

   println!("Enter empoloyee name: ");
    io::stdin().read_line(&mut input2).expect("Invalid input"); 
    let empol:char = input2.trim().parse().expect("Invalid input");

    
    if code == 7 {
        code_7();
        let mut file = OpenOptions::new().append(true).open("aigbon_juillet.txt").expect("File cannot open");
        file.write_all(b"\nAnalytics consulting services\nCustomer experience \nCybersecurity, strategy, risk, 
        compliance and resilience\nDigital transformation\nSupply chain and operations\nTechnology transformation");

        let mut file = OpenOptions::new().append(true).open("akpevwe_iloka.txt").expect("File cannot open");
        file.write_all(b"\nAudit services\nClimate change and sustainability services\nFinancial accounting advisory services\nForensic and 
        integrity services\nPrivate client audit experience\nAccounting Link\nAssurance");

    }
    else if code == 8{

        code_8();
        let mut file = OpenOptions::new().append(true).open("adamu_sagamu.txt").expect("File cannot open");
        file.write_all(b"\nTax planning\nTax function operations \nTax policy and controversy\nGlobal trade\nTax accounting\nTax compliance\nTransaction tax");

        let mut file = OpenOptions::new().append(true).open("gbenga_daniels.txt").expect("File cannot open");
        file.write_all(b"\nChange management and experience\nHR transformation \nIntegrated workforce mobility\nLearning and 
        development consulting\nRecognition and reward advisory\nWorkforce analytics\nPeople and workforce");
    }
    else if code ==9{
        code_9();
        let mut file = OpenOptions::new().append(true).open("ehis_ero.txt").expect("File cannot open");
        file.write_all(b"\nStrategy consulting\nCorporate and growth strategy \nTransaction strategy and execution\nRestructuring and turnaround strategy\nIndustry 
       strategy\nDigital business building\nCommercial strategy");

        let mut file = OpenOptions::new().append(true).open("maria_akinsola.txt").expect("File cannot open");
        file.write_all(b"\nCorporate finance\nDivestments and carve-outs\nSustainability and ESG Services\nM&A advisory\nM&A integration\nM&A
        technology and tools\nM&A advanced analytics");
    }

    else {
        println!("Sorry try again later");
    }

    if empol == 'C'{
      let mut file = std::fs::File::open("aigbon_juillet.txt").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap(); 
     print!("{}",contents);
    }
    if empol == 'A'{
      let mut file = std::fs::File::open("akpevwe_iloka.txt").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap(); 
     print!("{}",contents);
    }
    else if empol == 'T'{
      let mut file = std::fs::File::open("adamu_sagamu.txt").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap(); 
     print!("{}",contents);
    }
    else if empol == 'P'{
      let mut file = std::fs::File::open("gbenga_daniels.txt").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap(); 
     print!("{}",contents);
    }
    else if empol == 'N'{
      let mut file = std::fs::File::open("maria_akinsola.txt").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap(); 
     print!("{}",contents);
    }
    
    else if empol == 'S'{
      let mut file = std::fs::File::open("ehis_ero.txt").unwrap();
     let mut contents = String::new();
     file.read_to_string(&mut contents).unwrap(); 
     print!("{}",contents);
    }
    else {
        println!("Sorry try again");
    }
}