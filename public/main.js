
async function init(){
    let rustApp =null;
    const input =document.getElementById('upload');
    let fileBase64;
    try {
        rustApp =await import('../pkg'); 
        console.log(rustApp)
    } catch (e) {
        console.error(e);
        return;
    }
    let fileReader =new FileReader();
    input.addEventListener('change',()=>{
       // console.log(input.files[0]);
        fileReader.readAsDataURL(input.files[0]);

      
    });
    fileReader.onloadend=()=>{ 
        fileBase64=fileReader.result.replace(
            /^data:image\/(png|jpeg|jpg);base64,/,
            ''
        );
      let fileBase64Gray= rustApp.grayscale(fileBase64);   
       document.getElementById('new-img').setAttribute('src',fileBase64Gray);
       // console.log(fileBase64);
       let fileBase64Blur =rustApp.blur(fileBase64);
       document.getElementById('new-img-2').setAttribute('src',fileBase64Blur);
    }

}

init();