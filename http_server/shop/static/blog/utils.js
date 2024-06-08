let ad_directory = "/static/blog/ads/";

const ads = [
    {path: 'anvil.jpg', url: 'https://google.com', id: 'anvil'},
    {path: 'bidet.jpg', url: 'https://google.com', id: 'bidet'},
    {path: 'bliblie_control.jpg', url: 'https://google.com', id: 'bliblie'},
    {path: 'burger.jpeg', url: 'https://google.com', id: 'burger'},
    {path: 'goloneczka.jpg', url: 'https://google.com', id: 'goloneczka'},
    {path: 'micex.jpg', url: 'https://google.com', id: 'micex'},
    {path: 'mints.jpg', url: 'https://google.com', id: 'mints'},
    {path: 'nose_warmer.jpg', url: 'https://google.com', id: 'warmer'},
    {path: 'oranzenada.jpg', url: 'https://google.com', id: 'oranzenada'},
    {path: 'pizza.jpg', url: 'https://google.com', id: 'pizza'},
    {path: 'programerki.jpg', url: 'https://google.com', id: 'programerki'},
    {path: 'sciernisko.jpg', url: 'https://google.com', id: 'sciernisko'},
    {path: 'smalec.jpg', url: 'https://google.com', id: 'smalec'},
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
        a.href = './ad_click?link=' + encodeURIComponent(ad.url) + '&id=' + encodeURIComponent(ad.id);
        a.target = '_blank';
        element.appendChild(a);

        const img = document.createElement('img');
        img.src = ad_directory + ad.path;
        img.alt = 'Ad ' + ad.id;
        a.appendChild(img);
    });
}

document.addEventListener('DOMContentLoaded', displayRandomAds);
