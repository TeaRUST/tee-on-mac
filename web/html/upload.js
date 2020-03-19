$(()=>{
  $('#btn').click(()=>{
    let file = $('#file')[0].files[0];
    
    const fd = new FormData();
    // fd.append("name", "bill");
    fd.append('file', file);

    $.ajax({
      url: '/upload_wasm',
      type: 'post',
      data: fd,
      processData: false,
      contentType: false,
      success: function(rs){
        console.log(rs);
      }
    });

    

  })
})