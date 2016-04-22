var page = require('webpage').create();

page.viewportSize = { width: 1024, height: 768 };
page.open('http://localhost:8000/', function(status) {
    if (status !== 'success') {
        console.log('page load failed');
        phantom.exit(1);
    }
    page.render('index.png');
    phantom.exit();
});
