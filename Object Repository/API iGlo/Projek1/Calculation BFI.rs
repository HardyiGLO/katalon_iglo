<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Calculation BFI</name>
   <tag></tag>
   <elementGuidId>3e61520a-3a70-465a-a8fd-22754684e0b2</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\n{\n\&quot;product_calculation\&quot;: \&quot;ABF\&quot;,\n\&quot;otr\&quot;: 200000000,\n\&quot;unit_asset\&quot;: 1,\n\&quot;tenor\&quot;: 12,\n\&quot;uang_muka\&quot;: 0,\n\&quot;nilai_invoice\&quot;: \&quot;\&quot;,\n\&quot;nilai_invoice_di_biayai\&quot;: \&quot;\&quot;,\n\&quot;biaya_komitmen_pct\&quot;: \&quot;\&quot;,\n\&quot;skema_pencairan\&quot;: 1,\n\&quot;tipe_angsuran\&quot;: 0,\n\&quot;biaya_provisi_pct\&quot;: \&quot;0\&quot;,\n\&quot;biaya_notaris\&quot;: \&quot;0\&quot;,\n\&quot;biaya_fidusia\&quot;: \&quot;0\&quot;,\n\&quot;biaya_admin\&quot;: \&quot;2000000\&quot;,\n\&quot;biaya_survey\&quot;: \&quot;\&quot;,\n\&quot;biaya_lain\&quot;: \&quot;\&quot;,\n\&quot;grace_period\&quot;: \&quot;0\&quot;,\n\&quot;skema_pembiayaan\&quot;: \&quot;1\&quot;,\n\&quot;stepping\&quot;: [\n{\n\&quot;bulan\&quot;: 12,\n\&quot;porsi\&quot;: 100\n}\n],\n\&quot;pembayaran_biaya\&quot;: \&quot;\&quot;,\n\&quot;is_insurance\&quot;: 1,\n\&quot;tipe_asuransi\&quot;: \&quot;TLO\&quot;,\n\&quot;polis\&quot;: \&quot;0\&quot;,\n\&quot;metode_pembayaran_asuransi\&quot;: \&quot;Cash\&quot;,\n\&quot;discount_premi_asuransi\&quot;: \&quot;0\&quot;,\n\&quot;premi_rate\&quot;: \&quot;1.5\&quot;,\n\&quot;perluasan_banjir_rate\&quot;: [\n],\n\&quot;perluasan_gempa_rate\&quot;: [\n],\n\&quot;perluasan_srcc_rate\&quot;: [\n],\n\&quot;perluasan_teror_rate\&quot;: [\n],\n\&quot;tpl_rate\&quot;: \&quot;0\&quot;,\n\&quot;tpl_amount\&quot;: \&quot;0\&quot;,\n\&quot;perluasan_pa_driver_rate\&quot;: \&quot;0\&quot;,\n\&quot;pa_driver_amount\&quot;: \&quot;0\&quot;,\n\&quot;perluasan_pa_passenger_rate\&quot;: \&quot;0\&quot;,\n\&quot;pa_passenger_amount\&quot;: \&quot;0\&quot;,\n\&quot;pa_passenger_quantity\&quot;: \&quot;0\&quot;,\n\&quot;perluasan_legal_liabilites_rate\&quot;: \&quot;0\&quot;,\n\&quot;legal_liabilities_amount\&quot;: \&quot;0\&quot;,\n\&quot;eff_rate\&quot;: \&quot;20.009\&quot;,\n\&quot;depresiasi\&quot;: [\n\&quot;100\&quot;\n],\n\&quot;angsuran_irreguler\&quot;: [],\n\&quot;future_value\&quot;: \&quot;0\&quot;,\n\&quot;objek_pembiayaan\&quot;: \&quot;Truck\&quot;,\n\&quot;tahun_kendaraan\&quot;: \&quot;2022\&quot;,\n\&quot;biaya_loading_pct\&quot;: \&quot;\&quot;,\n\&quot;insurance_company\&quot;: \&quot;\&quot;,\n\&quot;rate_insurance_factoring\&quot;: \&quot;\&quot;,\n\&quot;biaya_admin_asuransi\&quot;: \&quot;\&quot;,\n\&quot;rumus_hitung_asuransi\&quot;: \&quot;\&quot;,\n\&quot;round\&quot;: 1\n}&quot;,
  &quot;contentType&quot;: &quot;text/html&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>45e2a673-3f8e-48c1-b6ca-cf89f1b76cfb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.0.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://34.128.87.8:83/getCalculation</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
