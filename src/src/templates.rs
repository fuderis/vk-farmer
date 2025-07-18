/// Generates bot profile HTML code
pub fn gen_bot_profile(replaces: Vec<(String, String)>) -> String {
    let mut s = r##"
        <div class="block" target="__ID__">
            <div class="header">
                <h4 class="name">__NAME__</h4>
                <button target="__ID__" class="delete-bot" title="Remove bot profile">
                    <img class="icon" src="../assets/images/icons/cross-icon.svg" alt="X">
                </button>
            </div>

            <form class="options">
                <div class="line input name" style="margin-top:0">
                    <label for="__ID__--name">Name:</label>
                    <input target="__ID__" id="__ID__--name" type="text" name="name" value="__NAME__" placeholder="Enter profile name" title="Enter profile name">
                </div>

                <div class="line input vk-id">
                    <label for="__ID__--vk-id">VK ID:</label>
                    <input target="__ID__" id="__ID__--vk-id" type="text" name="vk_id" value="__VK_ID__" placeholder="Enter VK profile ID" title="Enter VK profile ID">
                </div>

                <div class="line switcher likes">
                    <input target="__ID__" id="__ID__--farm-likes" type="checkbox" name="farm_likes" style="display:none" __FARM_LIKES__>
                    <label for="__ID__--farm-likes" title="Enable farming likes">
                        <div class="switcher"></div>
                        <span class="text">Farm likes</span>
                    </label>
                    <span class="limit">
                        <input target="__ID__" type="number" name="likes_limit" value="__LIKES_LIMIT__" title="Likes limit">
                        <div class="arrows">
                            <button class="input-plus" type="button">
                                <img class="icon" src="../assets/images/icons/arrow-up.svg" alt="+">
                            </button>
                            <button class="input-minus" type="button">
                                <img class="icon" src="../assets/images/icons/arrow-up.svg" alt="-">
                            </button>
                        </div>
                    </span>
                </div>

                <div class="line switcher friends">
                    <input target="__ID__" id="__ID__--farm-friends" type="checkbox" name="farm_friends" style="display:none" __FARM_FRIENDS__>
                    <label for="__ID__--farm-friends" title="Enable farming friends">
                        <div class="switcher"></div>
                        <span class="text">Farm friends</span>
                    </label>
                    <span class="limit">
                        <input target="__ID__" type="number" name="friends_limit" value="__FRIENDS_LIMIT__" title="Likes limit">
                        <div class="arrows">
                            <button class="input-plus" type="button">
                                <img class="icon" src="../assets/images/icons/arrow-up.svg" alt="+">
                            </button>
                            <button class="input-minus" type="button">
                                <img class="icon" src="../assets/images/icons/arrow-up.svg" alt="-">
                            </button>
                        </div>
                    </span>
                </div>

                <div class="line switcher subscribes">
                    <input target="__ID__" id="__ID__--farm-subscribes" type="checkbox" name="farm_subscribes" style="display:none" __FARM_SUBSCRIBES__>
                    <label for="__ID__--farm-subscribes" title="Enable farming subscribes">
                        <div class="switcher"></div>
                        <span class="text">Farm subscribes</span>
                    </label>
                    <span class="limit">
                        <input target="__ID__" type="number" name="subscribes_limit" value="__SUBSCRIBES_LIMIT__" title="Likes limit">
                        <div class="arrows">
                            <button class="input-plus" type="button">
                                <img class="icon" src="../assets/images/icons/arrow-up.svg" alt="+">
                            </button>
                            <button class="input-minus" type="button">
                                <img class="icon" src="../assets/images/icons/arrow-up.svg" alt="-">
                            </button>
                        </div>
                    </span>
                </div>
            </form>

            <div class="progress-bar">
                <p class="percent"><span class="value">0</span>%</p>
                <p class="descr">completed</p>
            </div>

            <button target="__ID__" class="primary start-farm" title="Start farming">Start farm</button>
            <button target="__ID__" class="secondary stop-farm" title="Stop farming">Stop farm</button>
        </div>
    "##.to_owned();

    for (k, v) in replaces {
        s = s.replace(&k, &v);
    }

    s
}
