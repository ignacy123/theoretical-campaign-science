const ads = [
    {path: 'ads/anvil.jpg', url: 'https://google.com'},
    {path: 'ads/bidet.jpg', url: 'https://google.com'},
    {path: 'ads/bliblie_control.jpg', url: 'https://google.com'},
    {path: 'ads/burger.jpeg', url: 'https://google.com'},
    {path: 'ads/goloneczka.jpg', url: 'https://google.com'},
    {path: 'ads/micex.jpg', url: 'https://google.com'},
    {path: 'ads/mints.jpg', url: 'https://google.com'},
    {path: 'ads/nose_warmer.jpg', url: 'https://google.com'},
    {path: 'ads/oranzenada.jpg', url: 'https://google.com'},
    {path: 'ads/pizza.jpg', url: 'https://google.com'},
    {path: 'ads/programerki.jpg', url: 'https://google.com'},
    {path: 'ads/sciernisko.jpg', url: 'https://google.com'},
    {path: 'ads/smalec.jpg', url: 'https://google.com'},
];

function getRandomAds(count) {
    let currentIndex = ads.length;

    while (currentIndex > ads.length - count) {
        let randomIndex = Math.floor(Math.random() * currentIndex);
        currentIndex -= 1;
        [ads[currentIndex], ads[randomIndex]] = [ads[randomIndex], ads[currentIndex]];
    }
    const result = ads.slice(ads.length - count, ads.length);
    console.log(result)
    return result;
}

function displayRandomAds() {
    const elements = [
        document.getElementById('ad1'),
        document.getElementById('ad2'),
        document.getElementById('ad3')
    ];
    const ads = getRandomAds(3);

    elements.forEach((element, index) => {
        const ad = ads[index];

        const a = document.createElement('a');
        a.href = ad.url;
        a.target = '_blank';
        element.appendChild(a);

        const img = document.createElement('img');
        img.src = ad.path;
        img.alt = `Ad ${index + 1}`;
        a.appendChild(img);
    });
}

document.addEventListener('DOMContentLoaded', displayRandomAds);
