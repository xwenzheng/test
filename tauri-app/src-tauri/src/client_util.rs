use serde::{Deserialize, Serialize};
use suanleme_sdk::{
    dto::{Config, Request},
    SuanlemeClient,
};

/// 创建并初始化 SuanlemeClient
pub fn create_suanleme_client() -> Result<SuanlemeClient, String> {
    // 这里也可以改成从环境变量或配置文件读取
    let rsa_pubk = "MIIBCgKCAQEAt3OjweXQ08fguv04Wxlu3hqlL1zgS7kZ1oF7B663raxAr1Sto5RImiYgYjIW1nL+7lel3jpYj29EE9GRoeZOYKyY3nEJmw4/lmxhHCbc19W+n+czpNlwaW8bGQ7pd7b38xuN4OZ8WiB+QpFQXv8aOZOLe636ySE8+XoSIL3GwyiUn0dhe4K4oZBU09t3rnFPRu0tTN9vmIa0JXGdtRa7KF+zRxIVvBxhViVtd5MXFDzATJcOcLxp/QGmslIOGBbN6RlQPYAcu9Dvr2V1PlcS+l00WtokBLiovJgW2wErkBjLYVOxIpnvydielSOZqsBnBa4gWO/4Nz8v2kUvY4XZKwIDAQAB"
        .to_string();
    let rsa_prik = "MIIEpAIBAAKCAQEAqS5gLu6yRQ51f0AyenzG4QKiMEclJG6jIhxM6q4wBWjcqEw9PHOdr65hmhmAIP+5tiJWQyQGX9j+rwl+lOVkElVGLgT1mzyls+B4kmii8EOjBB5TDeI9v7HQqDSO+6g4pVuhef5ooKyJLqzSbr4s8KIscavxHRL8YyTRwsyHj3UxWXvP4/Lzju0glO4ZboZCLRRbWS7lJ6dJfjVz4zeIwCVTSSu9/mIEUOoy7bXmosM4MfaZxsONViMjLvdHHAF0fW/bpT30ktHItSgnzn2/qZzUu33JnzaKoy6eZpbnpJvGtxFQdlpBkw9Ra5IfXEK9rh+BBfP7nTOvXT1o10CMswIDAQABAoIBAQCGn4aeEgb9eJH63mTMnznKl0PkUti1cuyRGiACpvVDeLwNmA0qa33Q7FxyOVSB7bwjuJMX/jSsVInlUxLM/PRyujSMLhoam1noFvka1/TAZ7Hydwb0B1HHloFJN/mHO8dHnfCSmOU85c6Lu63/vYkZOpNPKNlGn3MCmavo7MP6XKXHp9Ev+HuvT/uVxuF52kiPxzShqUjJrpplqNnD3FddSddwJGSH58TJpD5oFOIZWjq9xxTyUTbSuEO68Lf7oGmQlK9sABeVbxNm0N8Hy2i4sSnKpf3xif9MrcO+qt0A6qZNHzoPigN8YRLsrp1sXY1nU9o26vfOYhcsFosQD9RBAoGBANVJbDz8LXajl6gPjK/J5o7dR4yxWtZKLW2wemBlByS9z1FtWdr9sFIKbxEYlCrHQDBxBlB8jk5yinYqMFGe0vn3LjsFeQplS+l55T71KNKnwB6DsMFAKR89wb8zyvkuZWFgE+MxPK6rT8MzYiTfZk+y7/fi0JFU4VBomHMCn2sjAoGBAMsPybS9GE2WqBzm9So3XMKqoD35ybzA/NaCsl7cZoKnA65gsfw7qxrvsjM862kuKqPO3vlRFumrqy4ljoWBTZhlLckcBq04oiw2sdoR4VuJM5jknY+BmgbClNuaXKhkmdq2jx4f6lbgigq9gYtk5peav67xI9+ANJNzYlm22PkxAoGBAI7zoBUb3AjZSqs8iMnFY787NlppAH4Bx4LuNodnDyukAFEOHpx1TfkevyRROfWWCEKvblEBuot/n6flTO9XqQYakqTCXUcHb6KzrV/OBydxgYWyNATCUQ18YeAaAZ+TZiWmtI2gkZBsh5BIXy5hgPxH7ShSoAHYRw+SmyLyjhDLAoGAV16VpQRvfoLCFp1xAxcK/F85zzC4klRe9VUKtjISkUiXFuJX8nUh69cST2V4zKqmghCyc4dmWmgYoSRbrCm0X0u5ZXx/iuyBKpDl+TQUSLRB1RkYifRzd1Elh09larbOVAKUlZuZ0oOOIYzkPjvN05ErxHPQwYuCE6dju8ImnSECgYBo0fklDWeyLtkKSEoAgojF1fEoAISt/yKCcqED8H3syHn6To91Ho3pNNTRfdDu+SXasH0lPYRm1UFIRdnCzY6bu6AOp0uc0vQrDSng2mfRUlFaDS4Qg5mxZeNDscqpy6t74TBnupCKHMxDDKjWITP2kwhhwIuQa4R8tV/Ll18q0w=="
        .to_string();
    let version = "0.0.1".to_string();
    let token = "8f567568-4be0-4c38-b4d5-99eaa64e4a7d-20250409115305".to_string();

    let cfg = Config::default()
        .host("https://openapi.suanleme.vip".to_owned())
        .version(version)
        .rsa_pubk(rsa_pubk)
        .rsa_prik(rsa_prik)
        .token(token);

    SuanlemeClient::new(cfg)
        .map_err(|e| format!("初始化失败：{}", e))
}