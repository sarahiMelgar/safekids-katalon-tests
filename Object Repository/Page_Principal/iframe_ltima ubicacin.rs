<?xml version="1.0" encoding="UTF-8"?>
<WebElementEntity>
   <description></description>
   <name>iframe_ltima ubicacin</name>
   <tag></tag>
   <elementGuidId>e929ab35-a3f5-4c65-ba88-47cfb2feb88a</elementGuidId>
   <selectorCollection>
      <entry>
         <key>CSS</key>
         <value>[title=&quot;Última\ ubicación&quot;]</value>
      </entry>
      <entry>
         <key>XPATH</key>
         <value>//*[@title = 'Última ubicación']</value>
      </entry>
   </selectorCollection>
   <selectorMethod>XPATH</selectorMethod>
   <smartLocatorCollection>
      <entry>
         <key>SMART_LOCATOR</key>
         <value>iframe[title=&quot;Última ubicación&quot;]</value>
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
      <webElementGuid>63bc2e7e-433a-4591-9afc-d6335e7c8c30</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>title</name>
      <type>Main</type>
      <value>Última ubicación</value>
      <webElementGuid>30b28033-97ae-49ae-a842-3876dbce46c9</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>srcdoc</name>
      <type>Main</type>
      <value>&lt;!DOCTYPE html>
&lt;html>
&lt;head>
&lt;meta charset=&quot;utf-8&quot;>
&lt;meta name=&quot;referrer&quot; content=&quot;origin&quot;>
&lt;meta name=&quot;viewport&quot; content=&quot;width=device-width, initial-scale=1&quot;>
&lt;link rel=&quot;stylesheet&quot; href=&quot;https://unpkg.com/leaflet@1.9.4/dist/leaflet.css&quot;>
&lt;style>html,body,#m{height:100%;margin:0}.leaflet-tooltip{font:700 12px sans-serif;color:#2D1B3D}.leaflet-top.leaflet-right{top:50%;transform:translateY(-50%);margin-right:4px}&lt;/style>
&lt;/head>
&lt;body>
&lt;div id=&quot;m&quot;>&lt;/div>
&lt;script src=&quot;https://unpkg.com/leaflet@1.9.4/dist/leaflet.js&quot;>&lt;/script>
&lt;script>
var map = L.map('m', { zoomControl: false }).setView([19.399966803090162, -99.02997226934473], 16);
L.control.zoom({ position: 'topright' }).addTo(map);
L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
  maxZoom: 19, attribution: '&amp;copy; OpenStreetMap'
}).addTo(map);
var icono = L.divIcon({
  className: '',
  html: '&lt;div style=&quot;width:22px;height:22px;border-radius:50%;background:#9C4499;border:3px solid #fff;box-shadow:0 0 0 7px rgba(156,68,153,.25)&quot;>&lt;/div>',
  iconSize: [22, 22], iconAnchor: [11, 11]
});
var marcador = null, linea = null, circulo = null, extras = L.layerGroup().addTo(map);

window.addEventListener('message', function (e) {
  var d = e.data;
  if (!d || d.tipo !== 'mapa') return;
  var p = [d.lat, d.lng];

  if (!marcador) {
    marcador = L.marker(p, { icon: icono }).addTo(map);
    if (d.titulo) marcador.bindTooltip(d.titulo, { permanent: true, direction: 'top', offset: [0, -14] });
  } else {
    marcador.setLatLng(p);
    if (d.titulo &amp;&amp; marcador.getTooltip()) marcador.setTooltipContent(d.titulo);
  }

  if (d.ruta &amp;&amp; d.ruta.length > 1) {
    var pts = d.ruta.map(function (x) { return [x.latitud, x.longitud]; });
    if (!linea) linea = L.polyline(pts, { color: '#9C4499', weight: 4 }).addTo(map);
    else linea.setLatLngs(pts);
  } else if (linea) { map.removeLayer(linea); linea = null; }

  if (d.ajustar &amp;&amp; linea) map.fitBounds(linea.getBounds(), { padding: [30, 30] });
  else map.panTo(p);

  if (d.radio) {
    if (!circulo) circulo = L.circle(p, { radius: d.radio, color: '#9C4499', fillOpacity: 0.15 }).addTo(map);
    else { circulo.setLatLng(p); circulo.setRadius(d.radio); }
    } else if (circulo) { map.removeLayer(circulo); circulo = null; }

  extras.clearLayers();
  (d.marcadores || []).forEach(function (m) {
    L.circleMarker([m.lat, m.lng], { radius: 9, color: '#fff', weight: 2, fillColor: '#FF8C42', fillOpacity: 0.95 })
      .bindTooltip(m.texto, { permanent: true, direction: 'top', offset: [0, -8] })
      .addTo(extras);
  });

  if (d.encuadrar &amp;&amp; circulo) {
    var limites = circulo.getBounds();
    extras.eachLayer(function (l) { limites.extend(l.getLatLng()); });
    map.fitBounds(limites, { padding: [30, 30] });
  }
});
&lt;/script>
&lt;/body>
&lt;/html></value>
      <webElementGuid>f23e9955-61e5-4bbc-bba4-6d28b5980892</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>parent</name>
      <type>Main</type>
      <value>md5.v1-32c6b3ecc30279267003c21075db67e3</value>
      <webElementGuid>ef3c0f12-1055-4e0e-b470-cf660345b81a</webElementGuid>
   </webElementProperties>
   <webElementProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath</name>
      <type>Main</type>
      <value>//*[@title = 'Última ubicación']</value>
      <webElementGuid>d4d5b29d-3529-44a1-8dc0-150ed669acb6</webElementGuid>
   </webElementProperties>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:attributes</name>
      <type>Main</type>
      <value>//*[@title = 'Última ubicación']</value>
      <webElementGuid>57ca8d99-3430-43f5-9057-6ffaf0710a34</webElementGuid>
   </webElementXpaths>
   <webElementXpaths>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>xpath:customAttributes</name>
      <type>Main</type>
      <value>//iframe[@title = 'Última ubicación']</value>
      <webElementGuid>ead9a7cf-34cf-4af7-a9c9-967596fd77e5</webElementGuid>
   </webElementXpaths>
</WebElementEntity>
