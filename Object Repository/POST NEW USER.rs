<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST NEW USER</name>
   <tag></tag>
   <elementGuidId>f3e189ab-45fe-4e91-83a4-612ba33993be</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;first_name\&quot;: \&quot;${firstname}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${lastname}\&quot;,\n  \&quot;username\&quot;: \&quot;${username}\&quot;,\n  \&quot;job_position\&quot;: \&quot;${jobposition}\&quot;,\n  \&quot;job_level\&quot;: \&quot;${joblevel}\&quot;,\n  \&quot;salary\&quot;: ${salary},\n  \&quot;work_duration\&quot;: ${workduration}\n}\n&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>apikey</name>
      <type>Main</type>
      <value>${GlobalVariable.apiKey}</value>
      <webElementGuid>8eacb842-997c-43de-8e88-056c58ce648b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.authorization}</value>
      <webElementGuid>b5a5b3bc-64a9-4e9d-bf93-de6c5aa828c6</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Prefer</name>
      <type>Main</type>
      <value>return=representation</value>
      <webElementGuid>e4504ae5-0a96-4a8c-bda5-b646e98c9af4</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>a95bb6b1-ba32-44cc-9672-a46636ae9497</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.3.1</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.baseUrl}/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <validationSteps>
      <id>c8a58f51-ce2b-4bfe-ace3-323907d0b188</id>
      <name>POST NEW USER SCHEMA</name>
      <type>JSON_SCHEMA</type>
      <dataType>FILE</dataType>
      <target>RESPONSE</target>
      <data>E:\schema\mock api test\post new user schema.txt</data>
      <activate>true</activate>
   </validationSteps>
   <variables>
      <defaultValue>'Baby'</defaultValue>
      <description></description>
      <id>7abdf0ac-003e-468a-86b2-4bde2861706c</id>
      <masked>false</masked>
      <name>firstname</name>
   </variables>
   <variables>
      <defaultValue>'Groot'</defaultValue>
      <description></description>
      <id>4178264b-4199-444e-95cf-adac06ec692c</id>
      <masked>false</masked>
      <name>lastname</name>
   </variables>
   <variables>
      <defaultValue>'broot'</defaultValue>
      <description></description>
      <id>f2232229-8990-44cf-8e23-0e0f7aeb6fa3</id>
      <masked>false</masked>
      <name>username</name>
   </variables>
   <variables>
      <defaultValue>'QAE'</defaultValue>
      <description></description>
      <id>dd9259b0-940a-4b85-b5fe-96b74ca469b3</id>
      <masked>false</masked>
      <name>jobposition</name>
   </variables>
   <variables>
      <defaultValue>'Junior'</defaultValue>
      <description></description>
      <id>4262250e-6827-45cf-be3c-28d02044235d</id>
      <masked>false</masked>
      <name>joblevel</name>
   </variables>
   <variables>
      <defaultValue>7777777</defaultValue>
      <description></description>
      <id>a6ad19bd-d77d-4e87-b090-e81a9bdecc37</id>
      <masked>false</masked>
      <name>salary</name>
   </variables>
   <variables>
      <defaultValue>2</defaultValue>
      <description></description>
      <id>64cf7214-2b62-4ff9-9853-162308e1ffc3</id>
      <masked>false</masked>
      <name>workduration</name>
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


WS.verifyResponseStatusCode(response, 201)

assertThat(response.getStatusCode()).isEqualTo(201)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
