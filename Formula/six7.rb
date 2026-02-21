class Six7 < Formula
  desc "Secure P2P chat CLI with Six7 Protocol v1.3 - powered by Korium 0.7.29"
  homepage "https://github.com/six7chat/homebrew-six7"
  version "0.7.29"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/six7chat/homebrew-six7/releases/download/v0.7.29/six7-0.7.29-arm64-apple-darwin.tar.gz"
      sha256 "a5fbbb44ae807d3ead313e3ecd106cbc7fddc34eb27b302ccb3f21a6718a957d"
    end
  end

  def install
    bin.install "six7"
  end

  test do
    assert_match "six7", shell_output("#{bin}/six7 --version")
  end
end
