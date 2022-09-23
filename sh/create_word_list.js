/**
 * Must set uri!
 * Example: mongodb://username:password@localhost:27017/co
 * Sensitive Information! **DO NOT PUBLIC!**
 */
let db_uri = null;

if (db_uri === null && typeof(db_uri) !== "string") {
    throw new Error("Sensitive information <db_uri> has an illegal value!");
}

let db = connect( db_uri );

let bulk_arr = [];

console.log("Prepare Insertion ...")

let word_arr = [
    "测试", "成功", "太阳", "月亮", "星星", "海豹", "粒子", "光明", "黑暗", "老师",
    "侦探", "谋杀", "占卜", "宝剑", "猴子", "圣经", "嘴唇", "牛奶", "省长", "激素",
    "地狱", "网球", "绿色", "紫色", "血液", "黄色", "秃顶", "摸鱼", "咸鱼", "法力",
    "塔罗牌",
    "马里亚纳",
];

for (index in word_arr) {
    let word = word_arr[index];
    bulk_arr.push({
        updateOne: {
            "filter": { "word": word },
            "update": { $setOnInsert: { "_id": new ObjectId() , "word": word } },
            "upsert": true
        }
    });
}

console.log("Start Bulk Insertion ...")

db.words.bulkWrite(bulk_arr);

console.log("----------- End -----------")
