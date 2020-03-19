$(()=>{
  $('#btn').click(()=>{
    // let file = $('#file')[0].files[0];
    
    const fd = new FormData();
    let form = $('#form');
    fd.append('file', form.find('[name="file"]')[0].files[0]);
    fd.append('x', parseInt(form.find('[name="x"]').val(), 10) || 0);
    fd.append('y', parseInt(form.find('[name="y"]').val(), 10) || 0);

    // fd.append("name", "bill");

    // for (var [a, b] of fd.entries()) {
    //   console.log(a, b);
    // }    

    $.ajax({
      url: '/upload_wasm',
      type: 'post',
      data: fd,
      processData: false,
      contentType: false,
      success: function(rs){
        $('#rs').html('Result : '+rs);
      }
    });

    

  })
})