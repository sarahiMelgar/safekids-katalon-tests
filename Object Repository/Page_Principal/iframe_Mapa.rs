<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>iframe_Mapa</name>
   <tag></tag>
   <elementGuidId>fd79c607-41f8-40bd-a90c-76b0c0eab272</elementGuidId>
   <selectorCollection>
      <entry>
         <key>XPATH</key>
         <value>//*[@title = 'Mapa']</value>
      </entry>
      <entry>
         <key>CSS</key>
         <value>[title=&quot;Mapa&quot;]</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>iframe[title=&quot;Mapa&quot;]</value>
      </entry>
   </smartLocatorCollection>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>true</useRalativeImagePath>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>tag</name>
      <type>Main</type>
      <value>iframe</value>
      <webElementGuid>e6b8d949-27dd-443a-a25c-8e245877f442</webElementGuid>
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
      <webElementGuid>a74a3995-c7d7-4a88-ae80-ab52d57bd6d0</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>title</name>
      <type>Main</type>
      <value>Mapa</value>
      <webElementGuid>6d1470d8-fdec-4297-ac35-8ce6dc34f571</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-df56613c14958d21f7427badb4628ef1</value>
      <webElementGuid>98465167-bdf6-4158-ae8f-c594ef969458</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@title = 'Mapa']</value>
      <webElementGuid>dc145fc7-d6c2-4092-8fd0-28657d6e444b</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@title = 'Mapa']</value>
      <webElementGuid>43c483c5-f10d-4a8c-89f6-ae77dc6e750e</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//iframe[@title = 'Mapa']</value>
      <webElementGuid>712c4bd3-b87b-4825-b3a2-f829c2ff32ed</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
