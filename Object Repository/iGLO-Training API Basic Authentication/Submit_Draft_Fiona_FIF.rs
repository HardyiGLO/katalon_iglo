<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Submit_Draft_Fiona_FIF</name>
   <tag></tag>
   <elementGuidId>0f800711-440c-470f-9650-0fa6e169805b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>${Ltoken}</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;lob\&quot; : \&quot;MMU\&quot;,\n    \&quot;fullName\&quot; : \&quot;MARIYONO\&quot;,\n    \&quot;mobilePhone\&quot; : \&quot;081585598942\&quot;,\n    \&quot;noKtp\&quot; : \&quot;3173082011800008\&quot;,\n    \&quot;birthDate\&quot; : \&quot;1980-11-20\&quot;,\n    \&quot;birthPlace\&quot; : \&quot;JAKARTA\&quot;,\n    \&quot;gender\&quot; : \&quot;M\&quot;,\n    \&quot;mothersName\&quot; : \&quot;RODIAH\&quot;,\n    \&quot;financingType\&quot; : \&quot;U\&quot;,\n    \&quot;packageCode\&quot; : \&quot;PR0002\&quot;,\n    \&quot;tenor\&quot; : 12,\n    \&quot;downPayment\&quot; : 500000,\n    \&quot;channel\&quot; : \&quot;FNA\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>7b7cd16e-512b-49e5-a581-70fdc418d01c</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>*/*</value>
      <webElementGuid>040f8201-af5d-4cc3-9009-46998ac81998</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Connection</name>
      <type>Main</type>
      <value>keep-alive</value>
      <webElementGuid>45ae3319-b02f-417e-8c3e-db7132bc4148</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept-Encoding</name>
      <type>Main</type>
      <value>gzip, deflate, br</value>
      <webElementGuid>8878aec4-00f6-45e1-a46d-eb642ea28c3a</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer ${Ltoken}</value>
      <webElementGuid>192629a2-e58b-41e5-9a3c-50084ee36ece</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.1.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/leads/draft</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.token1</defaultValue>
      <description></description>
      <id>9df6d0f6-df2c-45aa-af1e-c16863009372</id>
      <masked>false</masked>
      <name>Ltoken</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

System.out.println (GlobalVariable.token1)

GlobalVariable.getdrafid = WS.getElementPropertyValue(response, 'result.draftId')
System.out.println (GlobalVariable.getdrafid)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
