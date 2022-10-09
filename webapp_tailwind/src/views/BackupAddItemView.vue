<div class="flex flex-col text-white max-h-screen h-screen select-none">
    <Title title="Add an Item"></Title>
    <!-- <div class="w-2/5 mx-auto first-letter:items-center"> -->
    <div class="wrapper mx-auto">

        <Header></Header>

        <div class="w-4/5 h-4/5 m-auto pt-4">
            <h1>{{current_collection.name}}</h1>

            <div class=" bg-purple-400 rounded-lg h-2"></div>
            <div class="text-center">
                <h1></h1>
            </div>

            <div class="form text-center p-4">
                <form class="space-y-5">
                    <div class="input-group">
                        <div>
                            <label for="name">Name</label>
                        </div>

                        <input type="text" class="field" id="name">
                    </div>
                    <div class="input-group">
                        <div>
                            <label for="description">Description</label>
                        </div>
                        <textarea type="text" style="height: 202px" class=" field" id="desription"></textarea>
                    </div>
                    <div class="input-group">
                        <div>
                            <label for="labels">Labels</label>
                        </div>
                        <select>
                            <option>teste</option>
                            <option>teste2</option>
                        </select>
                        <div class="inline px-4">
                            <button class="button">Add Label</button>
                        </div>
                        <div class="py-4">
                            <input type="text" class="field w-2/5" id="label">
                            <div class="inline px-4">
                                <button class="button">Add Custom Label</button>
                            </div>
                        </div>
                    </div>
                    <div class="labels">

                    </div>
                    <span class="button">Add Item</span>
                </form>
            </div>
        </div>
    </div>
</div>