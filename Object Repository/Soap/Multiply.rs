<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Multiply</name>
   <tag></tag>
   <elementGuidId>88d12bb5-567b-4d68-bb52-4b7eb45e27d9</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;Multiply xmlns=&quot;http://tempuri.org/&quot;>
            &lt;intA>${Num1}&lt;/intA>
            &lt;intB>${Num3}&lt;/intB>
        &lt;/Multiply>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>Multiply</soapServiceFunction>
   <variables>
      <defaultValue>'5'</defaultValue>
      <description></description>
      <id>af63e7bd-df18-4b40-866a-30121eab3eac</id>
      <masked>false</masked>
      <name>Num1</name>
   </variables>
   <variables>
      <defaultValue>'6'</defaultValue>
      <description></description>
      <id>61fc7e1f-b563-4654-994f-da4a1f3fa2c6</id>
      <masked>false</masked>
      <name>Num2</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.num</defaultValue>
      <description></description>
      <id>47db239e-927e-454e-b0b6-f7192dbed81c</id>
      <masked>false</masked>
      <name>Num3</name>
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
</verificationScript>
   <wsdlAddress>http://www.dneonline.com/calculator.asmx?WSDL</wsdlAddress>
</WebServiceRequestEntity>
