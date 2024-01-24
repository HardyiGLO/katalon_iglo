<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Token_Fiona_FIF</name>
   <tag></tag>
   <elementGuidId>4f86b349-abe4-4b29-bfdf-c36c58bbecbf</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userId\&quot;: \&quot;${user}\&quot;,\n    \&quot;password\&quot;: \&quot;${pass}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/x-www-form-urlencoded</value>
      <webElementGuid>8f285438-1154-4c58-9d0b-cdf9e5cb074d</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/user/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'fiona'</defaultValue>
      <description></description>
      <id>a7544bb7-76d5-48a0-84e7-98427f4f628e</id>
      <masked>false</masked>
      <name>user</name>
   </variables>
   <variables>
      <defaultValue>'zQe9Rk4qqvzs3Xyz'</defaultValue>
      <description></description>
      <id>9526f2ed-95fa-41e6-956c-7ab1ee4c593a</id>
      <masked>false</masked>
      <name>pass</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import java.nio.file.WatchService

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//GlobalVariable.token1 = WS.getElementPropertyValue(response, 'result.accessToken')
//System.out.println (GlobalVariable.token1)


GlobalVariable.token1 = WS.getElementPropertyValue(response, 'result.accessToken')
System.out.println(GlobalVariable.token1)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
