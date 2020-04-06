//! THIS IS A GENERATED FILE!
//! Take care when hand editing. Changes will be lost during subsequent runs of the code generator.
//!
//! version: 0.1.1
//!

#![allow(dead_code)]
#![allow(unused_imports)]
use std::io::{Read, Write};
use yaserde::{YaDeserialize, YaSerialize};

pub const SOAP_ENCODING: &str = "http://www.w3.org/2003/05/soap-encoding";
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
pub struct Header {}
#[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
#[yaserde(
    root = "Fault",
    namespace = "soapenv: http://schemas.xmlsoap.org/soap/envelope/",
    prefix = "soapenv"
)]
pub struct SoapFault {
    #[yaserde(rename = "faultcode", default)]
    pub fault_code: Option<String>,
    #[yaserde(rename = "faultstring", default)]
    pub fault_string: Option<String>,
}
pub type SoapResponse = Result<(reqwest::StatusCode, String), reqwest::Error>;

pub mod messages {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod types {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
    pub type SecureStore = XmlSecureStore;

    pub type User = XmlUser;

    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "users",
        namespace = "tns: http://xml.avaya.com/schema/import",
        namespace = "xsi: http://www.w3.org/2001/XMLSchema-instance",
        prefix = "nsi1"
    )]
    pub struct Users {
        #[yaserde(rename = "secureStore", default)]
        pub secure_store: Option<XmlSecureStore>,
        #[yaserde(rename = "user", default)]
        pub user: Vec<XmlUser>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "UserProvisionRules")]
    pub struct UserProvisionRules {
        #[yaserde(rename = "UserProvisionRuleName", default)]
        pub user_provision_rule_name: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "roles")]
    pub struct Roles {
        #[yaserde(rename = "role", default)]
        pub role: Vec<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "ownedContactLists")]
    pub struct OwnedContactLists {
        #[yaserde(rename = "contactList", default)]
        pub contact_list: XmlContactList,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "ownedContacts")]
    pub struct OwnedContacts {
        #[yaserde(rename = "contact", default)]
        pub contact: Vec<XmlContact>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlUser")]
    pub struct XmlUser {
        #[yaserde(rename = "UserOrganizationDetails", default)]
        pub user_organization_details: Option<UserOrganizationDetailsType>,
        #[yaserde(rename = "UserProvisionRules", default)]
        pub user_provision_rules: Option<UserProvisionRules>,
        #[yaserde(rename = "authenticationType", default)]
        pub authentication_type: String,
        #[yaserde(rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(rename = "displayName", default)]
        pub display_name: Option<String>,
        #[yaserde(rename = "displayNameAscii", default)]
        pub display_name_ascii: Option<String>,
        #[yaserde(rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(rename = "isDuplicatedLoginAllowed", default)]
        pub is_duplicated_login_allowed: Option<bool>,
        #[yaserde(rename = "isEnabled", default)]
        pub is_enabled: Option<bool>,
        #[yaserde(rename = "isVirtualUser", default)]
        pub is_virtual_user: Option<bool>,
        #[yaserde(rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(rename = "honorific", default)]
        pub honorific: Option<String>,
        #[yaserde(rename = "loginName", default)]
        pub login_name: String,
        #[yaserde(rename = "newLoginName", default)]
        pub new_login_name: Option<String>,
        #[yaserde(rename = "employeeNo", default)]
        pub employee_no: Option<String>,
        #[yaserde(rename = "department", default)]
        pub department: Option<String>,
        #[yaserde(rename = "organization", default)]
        pub organization: Option<String>,
        #[yaserde(rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "managerName", default)]
        pub manager_name: Option<String>,
        #[yaserde(rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(rename = "source", default)]
        pub source: Option<String>,
        #[yaserde(rename = "sourceUserKey", default)]
        pub source_user_key: Option<String>,
        #[yaserde(rename = "status", default)]
        pub status: Option<String>,
        #[yaserde(rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(rename = "surname", default)]
        pub surname: String,
        #[yaserde(rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(rename = "timeZone", default)]
        pub time_zone: Option<String>,
        #[yaserde(rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(rename = "userName", default)]
        pub user_name: Option<String>,
        #[yaserde(rename = "userPassword", default)]
        pub user_password: Option<String>,
        #[yaserde(rename = "commPassword", default)]
        pub comm_password: Option<String>,
        #[yaserde(rename = "userType", default)]
        pub user_type: Vec<String>,
        #[yaserde(rename = "roles", default)]
        pub roles: Option<Roles>,
        #[yaserde(rename = "localizedNames", default)]
        pub localized_names: Option<XmLocalizedNames>,
        #[yaserde(rename = "address", default)]
        pub address: Vec<XmlAddress>,
        #[yaserde(rename = "securityIdentity", default)]
        pub security_identity: Vec<XmlSecurityIdentity>,
        #[yaserde(rename = "ownedContactLists", default)]
        pub owned_contact_lists: Option<OwnedContactLists>,
        #[yaserde(rename = "ownedContacts", default)]
        pub owned_contacts: Option<OwnedContacts>,
        #[yaserde(rename = "presenceUserDefault", default)]
        pub presence_user_default: Option<XmlPresUserDefaultType>,
        #[yaserde(rename = "presenceUserACL", default)]
        pub presence_user_acl: Vec<XmlPresUserACLEntryType>,
        #[yaserde(rename = "presenceUserCLDefault", default)]
        pub presence_user_cl_default: Option<XmlPresUserCLDefaultType>,
        #[yaserde(rename = "commProfileSet", default)]
        pub comm_profile_set: Vec<XmlCommProfileSetType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlSecurityIdentity")]
    pub struct XmlSecurityIdentity {
        #[yaserde(rename = "identity", default)]
        pub identity: String,
        #[yaserde(rename = "realm", default)]
        pub realm: Option<String>,
        #[yaserde(rename = "type", default)]
        pub rs_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresInfoTypeAccessType")]
    pub struct XmlPresInfoTypeAccessType {
        #[yaserde(rename = "infoType", default)]
        pub info_type: XmlPresInfoTypeType,
        #[yaserde(rename = "access", default)]
        pub access: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresACRuleType")]
    pub struct XmlPresACRuleType {
        #[yaserde(rename = "infoTypeAccess", default)]
        pub info_type_access: Vec<XmlPresInfoTypeAccessType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresUserDefaultType")]
    pub struct XmlPresUserDefaultType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresUserCLDefaultType")]
    pub struct XmlPresUserCLDefaultType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresUserACLEntryType")]
    pub struct XmlPresUserACLEntryType {
        #[yaserde(flatten, default)]
        pub xml_pres_ac_rule_type: XmlPresACRuleType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlPresInfoTypeType")]
    pub struct XmlPresInfoTypeType {
        #[yaserde(rename = "label", default)]
        pub label: String,
        #[yaserde(rename = "filter", default)]
        pub filter: String,
        #[yaserde(rename = "specFlags", default)]
        pub spec_flags: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlContactList")]
    pub struct XmlContactList {
        #[yaserde(rename = "name", default)]
        pub name: String,
        #[yaserde(rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(rename = "members", default)]
        pub members: Vec<XmlContactListMember>,
        #[yaserde(rename = "contactListType", default)]
        pub contact_list_type: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlContactListMember")]
    pub struct XmlContactListMember {
        #[yaserde(rename = "isFavorite", default)]
        pub is_favorite: bool,
        #[yaserde(rename = "isSpeedDial", default)]
        pub is_speed_dial: bool,
        #[yaserde(rename = "speedDialEntry", default)]
        pub speed_dial_entry: Option<i32>,
        #[yaserde(rename = "isPresenceBuddy", default)]
        pub is_presence_buddy: bool,
        #[yaserde(rename = "label", default)]
        pub label: Option<String>,
        #[yaserde(rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(rename = "priorityLevel", default)]
        pub priority_level: Option<i32>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlContactAddress")]
    pub struct XmlContactAddress {
        #[yaserde(rename = "address", default)]
        pub address: String,
        #[yaserde(rename = "altLabel", default)]
        pub alt_label: Option<String>,
        #[yaserde(rename = "contactCategory", default)]
        pub contact_category: String,
        #[yaserde(rename = "contactType", default)]
        pub contact_type: String,
        #[yaserde(rename = "label", default)]
        pub label: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlAddress")]
    pub struct XmlAddress {
        #[yaserde(rename = "addressType", default)]
        pub address_type: String,
        #[yaserde(rename = "name", default)]
        pub name: String,
        #[yaserde(rename = "building", default)]
        pub building: Option<String>,
        #[yaserde(rename = "localityName", default)]
        pub locality_name: Option<String>,
        #[yaserde(rename = "postalCode", default)]
        pub postal_code: Option<String>,
        #[yaserde(rename = "room", default)]
        pub room: Option<String>,
        #[yaserde(rename = "stateOrProvince", default)]
        pub state_or_province: Option<String>,
        #[yaserde(rename = "country", default)]
        pub country: Option<String>,
        #[yaserde(rename = "street", default)]
        pub street: Option<String>,
        #[yaserde(rename = "businessphone", default)]
        pub businessphone: Option<String>,
        #[yaserde(rename = "otherbusinessphone", default)]
        pub otherbusinessphone: Option<String>,
        #[yaserde(rename = "fax", default)]
        pub fax: Option<String>,
        #[yaserde(rename = "homephone", default)]
        pub homephone: Option<String>,
        #[yaserde(rename = "otherhomephone", default)]
        pub otherhomephone: Option<String>,
        #[yaserde(rename = "mobilephone", default)]
        pub mobilephone: Option<String>,
        #[yaserde(rename = "othermobilephone", default)]
        pub othermobilephone: Option<String>,
        #[yaserde(rename = "pager", default)]
        pub pager: Option<String>,
        #[yaserde(rename = "pager2", default)]
        pub pager_2: Option<String>,
        #[yaserde(rename = "postalAddress", default)]
        pub postal_address: Option<String>,
        #[yaserde(rename = "isPrivate", default)]
        pub is_private: Option<bool>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlContact")]
    pub struct XmlContact {
        #[yaserde(rename = "company", default)]
        pub company: Option<String>,
        #[yaserde(rename = "description", default)]
        pub description: Option<String>,
        #[yaserde(rename = "displayName", default)]
        pub display_name: String,
        #[yaserde(rename = "displayNameAscii", default)]
        pub display_name_ascii: String,
        #[yaserde(rename = "dn", default)]
        pub dn: Option<String>,
        #[yaserde(rename = "givenName", default)]
        pub given_name: String,
        #[yaserde(rename = "givenNameAscii", default)]
        pub given_name_ascii: Option<String>,
        #[yaserde(rename = "initials", default)]
        pub initials: Option<String>,
        #[yaserde(rename = "middleName", default)]
        pub middle_name: Option<String>,
        #[yaserde(rename = "preferredGivenName", default)]
        pub preferred_given_name: Option<String>,
        #[yaserde(rename = "preferredLanguage", default)]
        pub preferred_language: Option<String>,
        #[yaserde(rename = "isPublic", default)]
        pub is_public: bool,
        #[yaserde(rename = "source", default)]
        pub source: String,
        #[yaserde(rename = "sourceUserKey", default)]
        pub source_user_key: String,
        #[yaserde(rename = "suffix", default)]
        pub suffix: Option<String>,
        #[yaserde(rename = "surname", default)]
        pub surname: String,
        #[yaserde(rename = "surnameAscii", default)]
        pub surname_ascii: Option<String>,
        #[yaserde(rename = "title", default)]
        pub title: Option<String>,
        #[yaserde(rename = "ContactAddress", default)]
        pub contact_address: Vec<XmlContactAddress>,
        #[yaserde(rename = "addresses", default)]
        pub addresses: Vec<XmlAddress>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlHandle")]
    pub struct XmlHandle {
        #[yaserde(rename = "handleName", default)]
        pub handle_name: String,
        #[yaserde(rename = "handleType", default)]
        pub handle_type: String,
        #[yaserde(rename = "handleSubType", default)]
        pub handle_sub_type: Option<String>,
        #[yaserde(rename = "domainName", default)]
        pub domain_name: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlCommProfileType")]
    pub struct XmlCommProfileType {
        #[yaserde(rename = "commProfileType", default)]
        pub comm_profile_type: String,
        #[yaserde(rename = "commProfileSubType", default)]
        pub comm_profile_sub_type: Option<String>,
        #[yaserde(rename = "jobId", default)]
        pub job_id: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "handleList")]
    pub struct HandleList {
        #[yaserde(rename = "handle", default)]
        pub handle: Vec<XmlHandle>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "commProfileList")]
    pub struct CommProfileList {
        #[yaserde(rename = "commProfile", default)]
        pub comm_profile: Vec<XmlCommProfileType>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlCommProfileSetType")]
    pub struct XmlCommProfileSetType {
        #[yaserde(rename = "commProfileSetName", default)]
        pub comm_profile_set_name: String,
        #[yaserde(rename = "isPrimary", default)]
        pub is_primary: bool,
        #[yaserde(rename = "handleList", default)]
        pub handle_list: Option<HandleList>,
        #[yaserde(rename = "commProfileList", default)]
        pub comm_profile_list: Option<CommProfileList>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "ForgeinCommProfileType")]
    pub struct ForgeinCommProfileType {
        #[yaserde(flatten, default)]
        pub xml_comm_profile_type: XmlCommProfileType,
        #[yaserde(prefix = "xsi", rename = "type", attribute)]
        pub xsi_type: String,
        #[yaserde(rename = "csEncryptionKeyId", default)]
        pub cs_encryption_key_id: Option<i64>,
        #[yaserde(rename = "servicePassword", default)]
        pub service_password: Option<String>,
        #[yaserde(rename = "serviceData", default)]
        pub service_data: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlSecureStore")]
    pub struct XmlSecureStore {
        #[yaserde(rename = "secureStoreData", default)]
        pub secure_store_data: String,
        #[yaserde(rename = "passwordEncrypted", default)]
        pub password_encrypted: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmlLocalizedName")]
    pub struct XmlLocalizedName {
        #[yaserde(rename = "locale", default)]
        pub locale: String,
        #[yaserde(rename = "name", default)]
        pub name: String,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "xmLocalizedNames")]
    pub struct XmLocalizedNames {
        #[yaserde(rename = "localizedName", default)]
        pub localized_name: Vec<XmlLocalizedName>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "tenant")]
    pub struct Tenant {
        #[yaserde(rename = "name", attribute)]
        pub name: String,
        #[yaserde(rename = "createTenantIfNotAlreadyPresent", attribute)]
        pub create_tenant_if_not_already_present: bool,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(root = "UserOrganizationDetailsType")]
    pub struct UserOrganizationDetailsType {
        #[yaserde(rename = "tenant", default)]
        pub tenant: Tenant,
        #[yaserde(rename = "organizationUnitLevelOne", default)]
        pub organization_unit_level_one: Option<String>,
        #[yaserde(rename = "organizationUnitLevelTwo", default)]
        pub organization_unit_level_two: Option<String>,
        #[yaserde(rename = "organizationUnitLevelThree", default)]
        pub organization_unit_level_three: Option<String>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlAgentProfile",
        namespace = "ns1: http://xml.avaya.com/schema/import_csm_agent",
        prefix = "ns1"
    )]
    pub struct XmlAgentProfile {
        #[yaserde(rename = "cmName", prefix = "ns1", default)]
        pub cm_name: String,
        #[yaserde(rename = "useExistingAgent", prefix = "ns1", default)]
        pub use_existing_agent: Option<bool>,
        #[yaserde(rename = "loginIdExtension", prefix = "ns1", default)]
        pub login_id_extension: String,
        #[yaserde(rename = "template", prefix = "ns1", default)]
        pub template: Option<String>,
        #[yaserde(rename = "securityCode", prefix = "ns1", default)]
        pub security_code: Option<String>,
        #[yaserde(rename = "aas", prefix = "ns1", default)]
        pub aas: Option<bool>,
        #[yaserde(rename = "audix", prefix = "ns1", default)]
        pub audix: Option<bool>,
        #[yaserde(rename = "password", prefix = "ns1", default)]
        pub password: Option<String>,
        #[yaserde(rename = "portExtension", prefix = "ns1", default)]
        pub port_extension: Option<String>,
        #[yaserde(rename = "deleteOnUnassign", prefix = "ns1", default)]
        pub delete_on_unassign: Option<bool>,
        #[yaserde(rename = "tn", prefix = "ns1", default)]
        pub tn: Option<i32>,
        #[yaserde(rename = "cor", prefix = "ns1", default)]
        pub cor: Option<i32>,
        #[yaserde(rename = "coveragePath", prefix = "ns1", default)]
        pub coverage_path: Option<String>,
        #[yaserde(rename = "lwcReception", prefix = "ns1", default)]
        pub lwc_reception: Option<String>,
        #[yaserde(rename = "lwcLogExternalCalls", prefix = "ns1", default)]
        pub lwc_log_external_calls: Option<bool>,
        #[yaserde(rename = "audixNameforMessaging", prefix = "ns1", default)]
        pub audix_namefor_messaging: Option<String>,
        #[yaserde(rename = "hearsServiceObservingTone", prefix = "ns1", default)]
        pub hears_service_observing_tone: Option<bool>,
        #[yaserde(rename = "loginIDforISDNSIPDisplay", prefix = "ns1", default)]
        pub login_i_dfor_isdnsip_display: Option<bool>,
        #[yaserde(rename = "autoAnswer", prefix = "ns1", default)]
        pub auto_answer: Option<String>,
        #[yaserde(rename = "miaAcrossSkills", prefix = "ns1", default)]
        pub mia_across_skills: Option<String>,
        #[yaserde(rename = "acwAgentConsideredIdle", prefix = "ns1", default)]
        pub acw_agent_considered_idle: Option<String>,
        #[yaserde(rename = "auxWorkReasonCodeType", prefix = "ns1", default)]
        pub aux_work_reason_code_type: Option<String>,
        #[yaserde(rename = "logoutReasonCodeType", prefix = "ns1", default)]
        pub logout_reason_code_type: Option<String>,
        #[yaserde(
            rename = "maximumTimeAgentInAcwBeforeLogoutSec",
            prefix = "ns1",
            default
        )]
        pub maximum_time_agent_in_acw_before_logout_sec: Option<String>,
        #[yaserde(rename = "forcedAgentLogoutTimeHr", prefix = "ns1", default)]
        pub forced_agent_logout_time_hr: Option<String>,
        #[yaserde(rename = "forcedAgentLogoutTimeSec", prefix = "ns1", default)]
        pub forced_agent_logout_time_sec: Option<String>,
        #[yaserde(rename = "directAgentSkill", prefix = "ns1", default)]
        pub direct_agent_skill: Option<String>,
        #[yaserde(rename = "callHandlingPreference", prefix = "ns1", default)]
        pub call_handling_preference: Option<String>,
        #[yaserde(rename = "serviceObjective", prefix = "ns1", default)]
        pub service_objective: Option<bool>,
        #[yaserde(rename = "directAgentCallsFirst", prefix = "ns1", default)]
        pub direct_agent_calls_first: Option<bool>,
        #[yaserde(rename = "localCallPreference", prefix = "ns1", default)]
        pub local_call_preference: Option<bool>,
        #[yaserde(rename = "skills", prefix = "ns1", default)]
        pub skills: Vec<XmlAgentLoginIdSkillsData>,
    }
    #[derive(Debug, Default, YaSerialize, YaDeserialize, Clone)]
    #[yaserde(
        root = "xmlAgentLoginIdSkillsData",
        namespace = "ns1: http://xml.avaya.com/schema/import_csm_agent",
        prefix = "ns1"
    )]
    pub struct XmlAgentLoginIdSkillsData {
        #[yaserde(rename = "number", prefix = "ns1", default)]
        pub number: String,
        #[yaserde(rename = "skillNumber", prefix = "ns1", default)]
        pub skill_number: String,
        #[yaserde(rename = "reserveLevel", prefix = "ns1", default)]
        pub reserve_level: Option<String>,
        #[yaserde(rename = "skillLevel", prefix = "ns1", default)]
        pub skill_level: Option<String>,
        #[yaserde(rename = "percentAllocation", prefix = "ns1", default)]
        pub percent_allocation: Option<String>,
    }
}

pub mod ports {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod bindings {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}

pub mod services {
    use super::*;
    use async_trait::async_trait;
    use yaserde::de::from_str;
    use yaserde::ser::to_string;
    use yaserde::{YaDeserialize, YaSerialize};
}
