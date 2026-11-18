<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>iframe_Mapa</name>
   <tag></tag>
   <elementGuidId>a20d613b-8a31-413c-a2bf-07b2de93305a</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>[title=&quot;Mapa&quot;]</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*[@title = 'Mapa']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>iframe</value>
      <webElementGuid>9cd7a1a2-d33d-49f2-9306-2c44024ab157</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>srcdoc</name>
      <type>Main</type>
      <value>&lt;!DOCTYPE html>&lt;html>&lt;head>
&lt;meta name=&quot;viewport&quot; content=&quot;width=device-width, initial-scale=1&quot;>
&lt;link rel=&quot;stylesheet&quot; href=&quot;https://unpkg.com/leaflet@1.9.4/dist/leaflet.css&quot;/>
&lt;style>html,body,#m{height:100%;margin:0}&lt;/style>&lt;/head>&lt;body>
&lt;div id=&quot;m&quot;>&lt;/div>
&lt;script src=&quot;https://unpkg.com/leaflet@1.9.4/dist/leaflet.js&quot;>&lt;/script>
&lt;script>
var map = L.map('m').setView([19.4, -99.03], 15);
L.tileLayer('https://tile.openstreetmap.org/{z}/{x}/{y}.png', {attribution: '© OpenStreetMap'}).addTo(map);
var marker = null;
function poner(la, lo, mover) {
  if (marker) marker.setLatLng([la, lo]); else marker = L.marker([la, lo]).addTo(map);
  if (mover) map.setView([la, lo], 16);
}
function enviar(o) {
  var s = JSON.stringify(o);
  if (window.ReactNativeWebView) window.ReactNativeWebView.postMessage(s);
  else window.parent.postMessage(s, '*');
}
map.on('click', function (e) {
  poner(e.latlng.lat, e.latlng.lng, false);
  enviar({ lat: e.latlng.lat, lng: e.latlng.lng });
});
window.addEventListener('message', function (ev) {
  try {
    var d = JSON.parse(ev.data);
    if (d.poner) poner(d.lat, d.lng, true);
  } catch (e) {}
});

&lt;/script>&lt;/body>&lt;/html></value>
      <webElementGuid>d19e7b4b-6de3-4106-89ab-d6fed766c70d</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>title</name>
      <type>Main</type>
      <value>Mapa</value>
      <webElementGuid>faea443e-8715-435d-b410-44c1af9ef616</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-df56613c14958d21f7427badb4628ef1</value>
      <webElementGuid>8d17641e-d50a-4357-853f-2757dcb30877</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@title = 'Mapa']</value>
      <webElementGuid>d03a01d6-fb39-4365-b81b-24252c4595ea</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@title = 'Mapa']</value>
      <webElementGuid>18139645-3180-4c90-9bbb-1ecb24845e63</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//iframe[@title = 'Mapa']</value>
      <webElementGuid>ad0b9c1d-10d3-4940-9f1d-b69ccaee9e85</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
